use crate::{
    fs::{FileType, Metadata},
    sys,
};
use std::{
    ffi, io,
    path::{Path, PathBuf},
};

/// Entries returned by the `ReadDir` iterator.
///
/// This corresponds to [`std::fs::DirEntry`].
///
/// Unlike `std::fs::DirEntry`, this API has no `DirEntry::path`, because
/// absolute paths don't interoperate well with the capability model.
///
/// And unlike `std::fs::DirEntry`, this API has a lifetime parameter.
///
/// Note that there is no `from_std` method, as `std::fs::DirEntry` doesn't
/// provide a way to construct a `DirEntry` without opening directories by
/// ambient paths.
///
/// [`std::fs::DirEntry`]: https://doc.rust-lang.org/std/fs/struct.DirEntry.html
pub struct DirEntry<'dir> {
    sys: sys::fs::DirEntry<'dir>,
}

impl<'dir> DirEntry<'dir> {
    /// Returns the path to the resource relative to `Dir` the `Dir::read_dir` was called from.
    ///
    /// This corresponds to [`std::fs::DirEntry::path`].
    ///
    /// [`std::fs::DirEntry::path`]: https://doc.rust-lang.org/std/fs/struct.DirEntry.html#method.path
    #[inline]
    pub fn path(&self) -> PathBuf {
        Path::new(".").join(self.file_name())
    }

    /// Returns the metadata for the file that this entry points at.
    ///
    /// This corresponds to [`std::fs::DirEntry::metadata`].
    ///
    /// [`std::fs::DirEntry::metadata`]: https://doc.rust-lang.org/std/fs/struct.DirEntry.html#method.metadata
    #[inline]
    pub fn metadata(&self) -> io::Result<Metadata> {
        self.sys.metadata()
    }

    /// Returns the file type for the file that this entry points at.
    ///
    /// This to [`std::fs::DirEntry::file_type`].
    ///
    /// [`std::fs::DirEntry::file_type`]: https://doc.rust-lang.org/std/fs/struct.DirEntry.html#method.file_type
    #[inline]
    pub fn file_type(&self) -> io::Result<FileType> {
        self.sys.file_type()
    }

    /// Returns the bare file name of this directory entry without any other leading path component.
    ///
    /// This corresponds to [`std::fs::DirEntry::file_name`].
    ///
    /// [`std::fs::DirEntry::file_name`]: https://doc.rust-lang.org/std/fs/struct.DirEntry.html#method.file_name
    #[inline]
    pub fn file_name(&self) -> ffi::OsString {
        self.sys.file_name()
    }
}

#[cfg(unix)]
impl<'dir> std::os::unix::fs::DirEntryExt for DirEntry<'dir> {
    fn ino(&self) -> u64 {
        self.sys.ino()
    }
}

// TODO: impl Debug for DirEntry? But don't expose DirEntry's path...
