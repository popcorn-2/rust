pub use crate::ffi::OsString as EnvKey;
use crate::ffi::{OsStr, OsString};
use crate::num::NonZero;
use crate::os::popcorn::ffi::{OsStrExt, OsStringExt};
use crate::path::Path;
use crate::sys::fs::File;
use crate::sys::pipe::Pipe;
use crate::sys::unsupported;
use super::env::{CommandEnv, CommandEnvs};
use crate::{fmt, io};
use alloc_crate::collections::BTreeMap;
use crate::os::popcorn::handle::{AsRawHandle, OwnedHandle, FromRawHandle, BorrowedHandle};
use crate::os::popcorn::proto::proc::{Thread, ThreadTr, Builder, BuilderTr};
use core::mem::ManuallyDrop;
use crate::process::StdioPipes;

////////////////////////////////////////////////////////////////////////////////
// Command
////////////////////////////////////////////////////////////////////////////////

pub struct Command {
    program: OsString,
    args: Vec<OsString>,
    env: CommandEnv,

    cwd: Option<OsString>,

    handles: BTreeMap<OsString, OwnedHandle<()>>, // adding the handle into the new process is a destructive move, so we don't want to destroy them on our side

    pending_error: io::Result<()>,

    is_stdin_null: bool,
    is_stdout_null: bool,
    is_stderr_null: bool,
}

#[derive(Debug)]
pub enum Stdio {
    Inherit,
    Null,
    MakePipe,
    FromClone(BorrowedHandle<'static, ()>),
    FromOwned(OwnedHandle<()>),
}

impl Stdio {
    fn clone_private(&self) -> io::Result<Self> {
        Ok(match self {
            Self::Inherit => Self::Inherit,
            Self::Null => Self::Null,
            Self::MakePipe => Self::MakePipe,
            Self::FromClone(handle) => Self::FromClone(handle.clone()),
            Self::FromOwned(handle) => Self::FromOwned(handle.try_clone()?),
        })
    }
}

fn dup_handle(name: &OsStr) -> io::Result<OwnedHandle<()>> {
    crate::os::popcorn::env::get_handle_untyped(name)
        .ok_or(io::Error::from_raw_os_error(8))?
        .try_clone_to_owned()
}

impl Command {
    pub fn new(program: &OsStr) -> Command {
        Command {
            program: program.to_owned(),
            args: vec![],
            env: Default::default(),
            cwd: None,
            handles: BTreeMap::new(),
            pending_error: Ok(()),
            is_stdin_null: false,
            is_stdout_null: false,
            is_stderr_null: false,
        }
    }

    pub fn arg(&mut self, arg: &OsStr) {
        self.args.push(arg.to_owned());
    }

    pub fn env_mut(&mut self) -> &mut CommandEnv {
        &mut self.env
    }

    pub fn cwd(&mut self, dir: &OsStr) {
        self.cwd = Some(dir.to_owned());
    }

    pub fn handle(&mut self, id: OsString, handle: io::Result<OwnedHandle<()>>) {
        match handle {
            Ok(handle) => { self.handles.insert(id, handle); },
            Err(e) => self.pending_error = Err(e),
        }
    }

    pub fn remove_handle(&mut self, id: &OsStr) {
        self.handles.remove(id);
    }

    pub fn stdin(&mut self, stdin: Stdio) {
        let res = match stdin {
            Stdio::Null => {
                self.is_stdin_null = true;
                self.remove_handle(OsStr::from_str("io.stdin"));
                return;
            },
            Stdio::MakePipe => todo!(),
            Stdio::Inherit => dup_handle(OsStr::from_str("io.stdin")),
            Stdio::FromClone(handle) => handle.try_clone_to_owned(),
            Stdio::FromOwned(handle) => Ok(handle),
        };
        self.handle(OsStr::from_str("io.stdin").to_owned(), res);
    }

    pub fn stdout(&mut self, stdout: Stdio) {
        let res = match stdout {
            Stdio::Null => {
                self.is_stdout_null = true;
                self.remove_handle(OsStr::from_str("io.stdout"));
                return;
            },
            Stdio::MakePipe => todo!(),
            Stdio::Inherit => dup_handle(OsStr::from_str("io.stdout")),
            Stdio::FromClone(handle) => handle.try_clone_to_owned(),
            Stdio::FromOwned(handle) => Ok(handle),
        };
        self.handle(OsStr::from_str("io.stdout").to_owned(), res);
    }

    pub fn stderr(&mut self, stderr: Stdio) {
        let res = match stderr {
            Stdio::Null => {
                self.is_stderr_null = true;
                self.remove_handle(OsStr::from_str("io.stderr"));
                return;
            },
            Stdio::MakePipe => todo!(),
            Stdio::Inherit => dup_handle(OsStr::from_str("io.stderr")),
            Stdio::FromClone(handle) => handle.try_clone_to_owned(),
            Stdio::FromOwned(handle) => Ok(handle),
        };
        self.handle(OsStr::from_str("io.stderr").to_owned(), res);
    }

    pub fn get_program(&self) -> &OsStr {
        &self.program
    }

    pub fn get_args(&self) -> CommandArgs<'_> {
        let mut iter = self.args.iter();
        iter.next();
        CommandArgs { iter }
    }

    pub fn get_envs(&self) -> CommandEnvs<'_> {
        self.env.iter()
    }
    
    pub fn get_env_clear(&self) -> bool {
        self.env.does_clear()
    }

    pub fn get_current_dir(&self) -> Option<&Path> {
        self.cwd.as_ref().map(|cs| Path::new(cs))
    }

    pub fn spawn(
        &mut self,
        default: Stdio,
        _needs_stdin: bool,
    ) -> io::Result<(Process, StdioPipes)> {
        if !self.handles.contains_key(OsStr::from_str("io.stdin")) && !self.is_stdin_null {
            self.stdin(default.clone_private()?);
        }

        if !self.handles.contains_key(OsStr::from_str("io.stdout")) && !self.is_stdout_null {
            self.stdout(default.clone_private()?);
        }

        if !self.handles.contains_key(OsStr::from_str("io.stderr")) && !self.is_stderr_null {
            self.stderr(default);
        }

        if let Err(e) = core::mem::replace(&mut self.pending_error, Ok(())) { return Err(e); }

        let handle = OwnedHandle::<Builder>::new(self.program.as_str(), Builder {})?;

        for (id, parent_handle) in self.handles.iter() {
			let parent_handle = parent_handle.try_clone()?;
            handle.add_handle(id.as_os_str(), parent_handle.as_raw_handle().0)?;
			core::mem::forget(parent_handle);
        }

        for (key, val) in self.env.capture() {
            let mut buf = <OsString as OsStringExt>::into_string(key);
            buf.push('=');
            buf.push_str(&<OsString as OsStringExt>::into_string(val));
            handle.add_env_var(&<OsString as OsStringExt>::from_string(buf))?;
        }

        handle.add_arg(&self.program)?;
        for arg in self.args.iter() {
            handle.add_arg(&arg)?;
        }

		let handle = handle.spawn()?;
        let res = Ok((
            Process { handle },
            StdioPipes {
                stdin: None,
                stdout: None,
                stderr: None,
            }
        ));
		res
    }
}

impl From<Pipe> for Stdio {
    fn from(pipe: Pipe) -> Stdio {
        pipe.diverge()
    }
}

impl From<io::Stdout> for Stdio {
    #[track_caller]
    fn from(_: io::Stdout) -> Stdio {
        Stdio::FromClone(crate::os::popcorn::env::get_handle_untyped("io.stdout").expect("stdout does not exist"))
    }
}

impl From<io::Stderr> for Stdio {
    #[track_caller]
    fn from(_: io::Stderr) -> Stdio {
        Stdio::FromClone(crate::os::popcorn::env::get_handle_untyped("io.stderr").expect("stderr does not exist"))
    }
}

impl From<File> for Stdio {
    fn from(file: File) -> Stdio {
        let file = ManuallyDrop::new(file);
        Stdio::FromOwned(unsafe { OwnedHandle::from_raw_handle(file.as_raw_handle()) })
    }
}

impl fmt::Debug for Command {
    // show all attributes
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            let mut debug_command = f.debug_struct("Command");
            debug_command.field("program", &self.program).field("args", &self.args);
            if !self.env.is_unchanged() {
                debug_command.field("env", &self.env);
            }

            if self.cwd.is_some() {
                debug_command.field("cwd", &self.cwd);
            }

            if let Some(stdin) = self.handles.get::<OsStr>(OsStrExt::from_str("io.stdin")) {
                debug_command.field("stdin", stdin);
            }
            if let Some(stdout) = self.handles.get::<OsStr>(OsStrExt::from_str("io.stdout")) {
                debug_command.field("stdout", stdout);
            }
            if let Some(stderr) = self.handles.get::<OsStr>(OsStrExt::from_str("io.stderr")) {
                debug_command.field("stderr", stderr);
            }

            debug_command.finish()
        } else {
            if let Some(ref cwd) = self.cwd {
                write!(f, "cd {cwd:?} && ")?;
            }
            if self.env.does_clear() {
                write!(f, "env -i ")?;
                // Altered env vars will be printed next, that should exactly work as expected.
            } else {
                // Removed env vars need the command to be wrapped in `env`.
                let mut any_removed = false;
                for (key, value_opt) in self.get_envs() {
                    if value_opt.is_none() {
                        if !any_removed {
                            write!(f, "env ")?;
                            any_removed = true;
                        }
                        write!(f, "-u {} ", key.to_string_lossy())?;
                    }
                }
            }
            // Altered env vars can just be added in front of the program.
            for (key, value_opt) in self.get_envs() {
                if let Some(value) = value_opt {
                    write!(f, "{}={value:?} ", key.to_string_lossy())?;
                }
            }
            if self.program != self.args[0] {
                write!(f, "[{:?}] ", self.program)?;
            }
            write!(f, "{:?}", self.args[0])?;

            for arg in &self.args[1..] {
                write!(f, " {:?}", arg)?;
            }
            Ok(())
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Default)]
#[non_exhaustive]
pub struct ExitStatus(isize);

impl ExitStatus {
    pub fn exit_ok(&self) -> Result<(), ExitStatusError> {
        if self.0 < 0 {
            Err(ExitStatusError(unsafe { NonZero::new_unchecked(self.0) }))
        } else {
            Ok(())
        }
    }

    pub fn code(&self) -> Option<i32> {
        Some(self.0 as i32)
    }
}

impl fmt::Display for ExitStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub struct ExitStatusError(NonZero<isize>);

impl Clone for ExitStatusError {
    fn clone(&self) -> ExitStatusError {
        Self(self.0)
    }
}

impl Copy for ExitStatusError {}

impl PartialEq for ExitStatusError {
    fn eq(&self, other: &ExitStatusError) -> bool {
        self.0 == other.0
    }
}

impl Eq for ExitStatusError {}

impl fmt::Debug for ExitStatusError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.get())
    }
}

impl Into<ExitStatus> for ExitStatusError {
    fn into(self) -> ExitStatus {
        ExitStatus(self.0.get())
    }
}

impl ExitStatusError {
    pub fn code(self) -> Option<NonZero<i32>> {
        todo!()
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct ExitCode(isize);

impl ExitCode {
    pub const SUCCESS: ExitCode = ExitCode(0);
    pub const FAILURE: ExitCode = ExitCode(-1);

    pub fn as_i32(&self) -> i32 {
        self.0 as i32
    }
}

impl From<u8> for ExitCode {
    fn from(code: u8) -> Self {
        Self(code as isize)
    }
}

pub struct Process {
    handle: OwnedHandle<Thread>,
}

impl Process {
    pub fn id(&self) -> u32 {
        self.handle.as_raw_handle().0 as u32
    }

    pub fn kill(&mut self) -> io::Result<()> {
        unsupported()
    }

    pub fn wait(&mut self) -> io::Result<ExitStatus> {
        let exit_code = self.handle.join()?;
        Ok(ExitStatus(exit_code))
    }

    pub fn try_wait(&mut self) -> io::Result<Option<ExitStatus>> {
        unsupported()
    }
}

pub struct CommandArgs<'a> {
    iter: crate::slice::Iter<'a, OsString>,
}

impl<'a> Iterator for CommandArgs<'a> {
    type Item = &'a OsStr;
    fn next(&mut self) -> Option<&'a OsStr> {
        self.iter.next().map(|os| &**os)
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a> ExactSizeIterator for CommandArgs<'a> {
    fn len(&self) -> usize {
        self.iter.len()
    }
    fn is_empty(&self) -> bool {
        self.iter.is_empty()
    }
}

impl<'a> fmt::Debug for CommandArgs<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.iter.clone()).finish()
    }
}

pub type ChildPipe = crate::sys::pipe::Pipe;

pub fn read_output(
    out: ChildPipe,
    _stdout: &mut Vec<u8>,
    _err: ChildPipe,
    _stderr: &mut Vec<u8>,
) -> io::Result<()> {
    match out.diverge() {}
}

pub fn getpid() -> u32 {
    panic!("no pids on this platform")
}
