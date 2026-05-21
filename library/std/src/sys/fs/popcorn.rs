#![allow(dead_code)]

use crate::ffi::OsString;
use crate::fmt;
use crate::io::{self, BorrowedCursor, Error, IoSlice, IoSliceMut, SeekFrom};
use crate::path::{Path, PathBuf};
use crate::sys::time::SystemTime;
use crate::sys::unsupported;
use crate::os::popcorn::io::{OwnedHandle, FromRawHandle, RawHandle, AsRawHandle, IntoRawHandle, BorrowedHandle, AsHandle};
use crate::os::popcorn::proto::{io::Read as AbiRead, io::Write as AbiWrite};
use core::hash::{Hash, Hasher};
use core::io::BorrowedBuf;
use crate::fs::TryLockError;
use crate::sys::{FromInner, AsInner};
use crate::os::popcorn::fs::DirExt;

pub use crate::sys::fs::common::{copy, remove_dir_all};

mod dir;
pub use dir::Dir;

pub struct File {
    handle: OwnedHandle,
}

impl FromInner<OwnedHandle<&dyn AbiRead>> for File {
    fn from_inner(handle: OwnedHandle<&dyn AbiRead>) -> Self {
        Self { handle: handle.type_erase() }
    }
}

impl FromInner<OwnedHandle<&dyn AbiWrite>> for File {
    fn from_inner(handle: OwnedHandle<&dyn AbiWrite>) -> Self {
        Self { handle: handle.type_erase() }
    }
}

impl FromInner<OwnedHandle<(&dyn AbiRead, &dyn AbiWrite)>> for File {
    fn from_inner(handle: OwnedHandle<(&dyn AbiRead, &dyn AbiWrite)>) -> Self {
        Self { handle: handle.type_erase() }
    }
}

impl FromInner<OwnedHandle<(&dyn AbiWrite, &dyn AbiRead)>> for File {
    fn from_inner(handle: OwnedHandle<(&dyn AbiWrite, &dyn AbiRead)>) -> Self {
        Self { handle: handle.type_erase() }
    }
}

impl AsHandle<&'static dyn AbiRead> for File {
    fn as_handle(&self) -> BorrowedHandle<'_, &'static dyn AbiRead> {
        self.handle.as_handle().force_protocol()
    }
}

impl AsHandle<&'static dyn AbiWrite> for File {
    fn as_handle(&self) -> BorrowedHandle<'_, &'static dyn AbiWrite> {
        self.handle.as_handle().force_protocol()
    }
}

impl AsHandle<(&'static dyn AbiRead, &'static dyn AbiWrite)> for File {
    fn as_handle(&self) -> BorrowedHandle<'_, (&'static dyn AbiRead, &'static dyn AbiWrite)> {
        self.handle.as_handle().force_protocol()
    }
}

impl AsHandle<(&'static dyn AbiWrite, &'static dyn AbiRead)> for File {
    fn as_handle(&self) -> BorrowedHandle<'_, (&'static dyn AbiWrite, &'static dyn AbiRead)> {
        self.handle.as_handle().force_protocol()
    }
}

impl AsRawHandle for File {
    fn as_raw_handle(&self) -> RawHandle {
        self.handle.as_raw_handle()
    }
}

impl IntoRawHandle for File {
    fn into_raw_handle(self) -> RawHandle {
        self.handle.into_raw_handle()
    }
}

impl FromRawHandle for File {
    unsafe fn from_raw_handle(raw: RawHandle) -> Self {
        File { handle: unsafe { OwnedHandle::from_raw_handle(raw) } }
    }
}

pub struct FileAttr(!);

pub struct ReadDir(!);

pub struct DirEntry(!);

#[derive(Clone, Debug)]
pub struct OpenOptions {
    // generic
    read: bool,
    write: bool,
    append: bool,
    truncate: bool,
    create: bool,
    create_new: bool,
}

#[derive(Copy, Clone, Debug, Default)]
pub struct FileTimes {}

pub struct FilePermissions(!);

pub struct FileType(!);

#[derive(Debug)]
pub struct DirBuilder {}

impl FileAttr {
    pub fn size(&self) -> u64 {
        self.0
    }

    pub fn perm(&self) -> FilePermissions {
        self.0
    }

    pub fn file_type(&self) -> FileType {
        self.0
    }

    pub fn modified(&self) -> io::Result<SystemTime> {
        self.0
    }

    pub fn accessed(&self) -> io::Result<SystemTime> {
        self.0
    }

    pub fn created(&self) -> io::Result<SystemTime> {
        self.0
    }
}

impl Clone for FileAttr {
    fn clone(&self) -> FileAttr {
        self.0
    }
}

impl FilePermissions {
    pub fn readonly(&self) -> bool {
        self.0
    }

    pub fn set_readonly(&mut self, _readonly: bool) {
        self.0
    }
}

impl Clone for FilePermissions {
    fn clone(&self) -> FilePermissions {
        self.0
    }
}

impl PartialEq for FilePermissions {
    fn eq(&self, _other: &FilePermissions) -> bool {
        self.0
    }
}

impl Eq for FilePermissions {}

impl fmt::Debug for FilePermissions {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0
    }
}

impl FileTimes {
    pub fn set_accessed(&mut self, _t: SystemTime) {}
    pub fn set_modified(&mut self, _t: SystemTime) {}
}

impl FileType {
    pub fn is_dir(&self) -> bool {
        self.0
    }
    pub fn is_file(&self) -> bool {
        self.0
    }
    pub fn is_symlink(&self) -> bool {
        self.0
    }
}

impl Clone for FileType {
    fn clone(&self) -> FileType {
        self.0
    }
}

impl Copy for FileType {}

impl PartialEq for FileType {
    fn eq(&self, _other: &FileType) -> bool {
        self.0
    }
}

impl Eq for FileType {}

impl Hash for FileType {
    fn hash<H: Hasher>(&self, _h: &mut H) {
        self.0
    }
}

impl fmt::Debug for FileType {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0
    }
}

impl fmt::Debug for ReadDir {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0
    }
}

impl Iterator for ReadDir {
    type Item = io::Result<DirEntry>;

    fn next(&mut self) -> Option<io::Result<DirEntry>> {
        self.0
    }
}

impl DirEntry {
    pub fn path(&self) -> PathBuf {
        self.0
    }

    pub fn file_name(&self) -> OsString {
        self.0
    }

    pub fn metadata(&self) -> io::Result<FileAttr> {
        self.0
    }

    pub fn file_type(&self) -> io::Result<FileType> {
        self.0
    }
}

impl OpenOptions {
    pub fn new() -> OpenOptions {
        OpenOptions {
            // generic
            read: false,
            write: false,
            append: false,
            truncate: false,
            create: false,
            create_new: false,
        }
    }

    pub fn read(&mut self, read: bool) {
        self.read = read;
    }

    pub fn write(&mut self, write: bool) {
        self.write = write;
    }

    pub fn append(&mut self, append: bool) {
        self.append = append;
    }

    pub fn truncate(&mut self, truncate: bool) {
        self.truncate = truncate;
    }

    pub fn create(&mut self, create: bool) {
        self.create = create;
    }

    pub fn create_new(&mut self, create_new: bool) {
        self.create_new = create_new;
    }
}

impl File {
    #[expect(unused)]
    pub fn open(path: &Path, opts: &OpenOptions) -> io::Result<File> {
        let create = match (opts.create, opts.create_new) {
            (_, true) => 1,
            (true, false) => 2,
            (false, false) => 0,
        };

        let cwd = crate::fs::Dir::working_dir().ok_or(io::Error::new(
            io::ErrorKind::PermissionDenied, // does this make sense here? if we don't have a cwd
                                             // that just means the executable has no real fs perms
            Box::new(crate::os::popcorn::io::HandleNotFoundError(())), // feels like this could be improved
        ))?;

        cwd.as_inner().open_file(path, opts)
    }

    pub fn file_attr(&self) -> io::Result<FileAttr> {
        unsupported()
    }

    pub fn fsync(&self) -> io::Result<()> {
        self.flush()
    }

    pub fn datasync(&self) -> io::Result<()> {
        self.flush()
    }

    pub fn lock(&self) -> io::Result<()> {
        unsupported()
    }

    pub fn lock_shared(&self) -> io::Result<()> {
        unsupported()
    }

    pub fn try_lock(&self) -> Result<(), TryLockError> {
        unsupported().map_err(TryLockError::Error)
    }

    pub fn try_lock_shared(&self) -> Result<(), TryLockError> {
        unsupported().map_err(TryLockError::Error)
    }

    pub fn unlock(&self) -> io::Result<()> {
        unsupported()
    }

    pub fn truncate(&self, _size: u64) -> io::Result<()> {
        unsupported()
    }

    pub fn read(&self, buf: &mut [u8]) -> io::Result<usize> {
        let mut buf = BorrowedBuf::from(&mut *buf);
        self.read_buf(buf.unfilled())?;
        Ok(buf.len())
    }

    pub fn read_buf(&self, cursor: BorrowedCursor<'_>) -> io::Result<()> {
        self.handle.try_as::<&dyn AbiRead>()
            .ok_or(Error::from_raw_os_error(3))?
            .read(cursor)
            .map(|_| ())
    }

    pub fn read_vectored(&self, bufs: &mut [IoSliceMut<'_>]) -> io::Result<usize> {
        crate::io::default_read_vectored(|buf| self.read(buf), bufs)
    }

    pub fn is_read_vectored(&self) -> bool {
        false
    }

    pub fn write(&self, buf: &[u8]) -> io::Result<usize> {
        self.handle.try_as::<&dyn AbiWrite>()
            .ok_or(Error::from_raw_os_error(3))?
            .write(buf)
    }

    pub fn write_vectored(&self, bufs: &[IoSlice<'_>]) -> io::Result<usize> {
        crate::io::default_write_vectored(|buf| self.write(buf), bufs)
    }

    pub fn is_write_vectored(&self) -> bool {
        false
    }

    pub fn flush(&self) -> io::Result<()> {
        unsupported()
    }

    pub fn seek(&self, _pos: SeekFrom) -> io::Result<u64> {
        unsupported()
    }

    pub fn size(&self) -> Option<io::Result<u64>> {
        Some(unsupported())
    }

    pub fn tell(&self) -> io::Result<u64> {
        unsupported()
    }

    pub fn duplicate(&self) -> io::Result<File> {
        unsupported()
    }

    pub fn set_permissions(&self, _perm: FilePermissions) -> io::Result<()> {
        unsupported()
    }

    pub fn set_times(&self, _times: FileTimes) -> io::Result<()> {
        unsupported()
    }
}

impl DirBuilder {
    pub fn new() -> DirBuilder {
        DirBuilder {}
    }

    pub fn mkdir(&self, _p: &Path) -> io::Result<()> {
        unsupported()
    }
}

impl fmt::Debug for File {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("File").field("handle", &self.handle).finish()
    }
}

pub fn readdir(_p: &Path) -> io::Result<ReadDir> {
    unsupported()
}

pub fn unlink(_p: &Path) -> io::Result<()> {
    unsupported()
}

pub fn rename(_old: &Path, _new: &Path) -> io::Result<()> {
    unsupported()
}

pub fn set_perm(_p: &Path, perm: FilePermissions) -> io::Result<()> {
    match perm.0 {}
}

pub fn set_times(_p: &Path, _times: FileTimes) -> io::Result<()> {
    unsupported()
}

pub fn set_times_nofollow(_p: &Path, _times: FileTimes) -> io::Result<()> {
    unsupported()
}

pub fn rmdir(_p: &Path) -> io::Result<()> {
    unsupported()
}

pub fn exists(path: &Path) -> io::Result<bool> {
    let opts = OpenOptions::new();
    match File::open(path, &opts) {
        Ok(_) => Ok(true),
        Err(e) => match e.kind() {
            // The file definitely does not exist
            io::ErrorKind::NotFound => Ok(false),
            _ => Err(e),
        }
    }
}

pub fn readlink(_p: &Path) -> io::Result<PathBuf> {
    unsupported()
}

pub fn symlink(_original: &Path, _link: &Path) -> io::Result<()> {
    // This target doesn't support symlinks
    unsupported()
}

pub fn link(_src: &Path, _dst: &Path) -> io::Result<()> {
    // This target doesn't support symlinks
    unsupported()
}

pub fn stat(p: &Path) -> io::Result<FileAttr> {
    // This target doesn't support symlinks
    lstat(p)
}

pub fn lstat(_p: &Path) -> io::Result<FileAttr> {
    unsupported()
}

pub fn canonicalize(_p: &Path) -> io::Result<PathBuf> {
    unsupported()
}
