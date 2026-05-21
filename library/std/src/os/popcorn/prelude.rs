/// A prelude for conveniently writing platform-specific code.
///
/// Includes all extension traits, and some important type definitions.

pub use super::ffi::OsStrExt;
pub use super::ffi::OsStringExt;
pub use super::fs::DirExt;
pub use super::io::AsHandle;
pub use super::io::BorrowedHandle;
pub use super::io::FromRawHandle;
pub use super::io::IntoRawHandle;
pub use super::io::OwnedHandle;
pub use super::io::AsRawHandle;
pub use super::io::RawHandle;
pub use super::process::CommandExt;
