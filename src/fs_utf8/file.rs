#[cfg(unix)]
use std::os::unix::io::{AsRawFd, FromRawFd, IntoRawFd, RawFd};
#[cfg(windows)]
use std::os::windows::io::{AsRawHandle, FromRawHandle, IntoRawHandle, RawHandle};
use std::{fs, io, process};

/// A reference to an open file on a filesystem.
///
/// This corresponds to [`std::fs::File`].
///
/// Note that this `File` has no `open` or `create` methods. To open or create
/// a file, you must first obtain a [`Dir`] containing the path, and then call
/// [`Dir::open_file`] or [`Dir::create_file`].
///
/// [`std::fs::File`]: https://doc.rust-lang.org/std/fs/struct.File.html
/// [`Dir`]: struct.Dir.html
/// [`Dir::open_file`]: struct.Dir.html#method.open_file
/// [`Dir::create_file`]: struct.Dir.html#method.create_file
pub struct File {
    cap_std: crate::fs::File,
}

impl File {
    /// Constructs a new instance of `Self` from the given `std::fs::File`.
    #[inline]
    pub fn from_std(std: fs::File) -> Self {
        Self::from_cap_std(crate::fs::File::from_std(std))
    }

    /// Constructs a new instance of `Self` from the given `cap_std::fs::File`.
    #[inline]
    pub fn from_cap_std(cap_std: crate::fs::File) -> Self {
        Self { cap_std }
    }

    /// Attempts to sync all OS-internal metadata to disk.
    ///
    /// This corresponds to [`std::fs::File::sync_all`].
    ///
    /// [`std::fs::File::sync_all`]: https://doc.rust-lang.org/std/fs/struct.File.html#method.sync_all
    #[inline]
    pub fn sync_all(&self) -> io::Result<()> {
        self.cap_std.sync_all()
    }

    /// This function is similar to `sync_all`, except that it may not synchronize
    /// file metadata to a filesystem.
    ///
    /// This corresponds to [`std::fs::File::sync_data`].
    ///
    /// [`std::fs::File::sync_data`]: https://doc.rust-lang.org/std/fs/struct.File.html#method.sync_data
    #[inline]
    pub fn sync_data(&self) -> io::Result<()> {
        self.cap_std.sync_data()
    }

    /// Truncates or extends the underlying file, updating the size of this file
    /// to become size.
    ///
    /// This corresponds to [`std::fs::File::set_len`].
    ///
    /// [`std::fs::File::set_len`]: https://doc.rust-lang.org/std/fs/struct.File.html#method.set_len
    #[inline]
    pub fn set_len(&self, size: u64) -> io::Result<()> {
        self.cap_std.set_len(size)
    }

    /// Queries metadata about the underlying file.
    ///
    /// This corresponds to [`std::fs::File::metadata`].
    ///
    /// [`std::fs::File::metadata`]: https://doc.rust-lang.org/std/fs/struct.File.html#method.metadata
    #[inline]
    pub fn metadata(&self) -> io::Result<fs::Metadata> {
        self.cap_std.metadata()
    }

    /// Creates a new `File` instance that shares the same underlying file handle as the existing
    /// `File` instance.
    ///
    /// This corresponds to [`std::fs::File::try_clone`].
    ///
    /// [`std::fs::File::try_clone`]: https://doc.rust-lang.org/std/fs/struct.File.html#method.try_clone
    #[inline]
    pub fn try_clone(&self) -> io::Result<Self> {
        Ok(Self::from_cap_std(self.cap_std.try_clone()?))
    }
}

#[cfg(unix)]
impl FromRawFd for File {
    #[inline]
    unsafe fn from_raw_fd(fd: RawFd) -> Self {
        Self::from_std(fs::File::from_raw_fd(fd))
    }
}

#[cfg(windows)]
impl FromRawHandle for File {
    #[inline]
    unsafe fn from_raw_handle(handle: RawHandle) -> Self {
        Self::from_std(fs::File::from_raw_handle(handle))
    }
}

#[cfg(unix)]
impl AsRawFd for File {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        self.cap_std.as_raw_fd()
    }
}

#[cfg(windows)]
impl AsRawHandle for File {
    #[inline]
    fn as_raw_handle(&self) -> RawHandle {
        self.cap_std.as_raw_handle()
    }
}

#[cfg(unix)]
impl IntoRawFd for File {
    #[inline]
    fn into_raw_fd(self) -> RawFd {
        self.cap_std.into_raw_fd()
    }
}

#[cfg(windows)]
impl IntoRawHandle for File {
    #[inline]
    fn into_raw_handle(self) -> RawHandle {
        self.cap_std.into_raw_handle()
    }
}

impl io::Read for File {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.cap_std.read(buf)
    }

    #[inline]
    fn read_vectored(&mut self, bufs: &mut [io::IoSliceMut]) -> io::Result<usize> {
        self.cap_std.read_vectored(bufs)
    }

    #[inline]
    fn read_exact(&mut self, buf: &mut [u8]) -> io::Result<()> {
        self.cap_std.read_exact(buf)
    }

    #[inline]
    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> io::Result<usize> {
        self.cap_std.read_to_end(buf)
    }

    #[inline]
    fn read_to_string(&mut self, buf: &mut String) -> io::Result<usize> {
        self.cap_std.read_to_string(buf)
    }

    // TODO: nightly-only APIs initializer?
}

impl io::Write for File {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.cap_std.write(buf)
    }

    #[inline]
    fn flush(&mut self) -> io::Result<()> {
        self.cap_std.flush()
    }

    #[inline]
    fn write_vectored(&mut self, bufs: &[io::IoSlice]) -> io::Result<usize> {
        self.cap_std.write_vectored(bufs)
    }

    #[inline]
    fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        self.cap_std.write_all(buf)
    }
}

impl io::Seek for File {
    #[inline]
    fn seek(&mut self, pos: io::SeekFrom) -> io::Result<u64> {
        self.cap_std.seek(pos)
    }

    // TODO: nightly-only APIs stream_len, stream_position?
}

impl From<File> for process::Stdio {
    #[inline]
    fn from(file: File) -> Self {
        From::<crate::fs::File>::from(file.cap_std)
    }
}

#[cfg(unix)]
impl std::os::unix::fs::FileExt for File {
    #[inline]
    fn read_at(&self, buf: &mut [u8], offset: u64) -> io::Result<usize> {
        self.cap_std.read_at(buf, offset)
    }

    #[inline]
    fn write_at(&self, buf: &[u8], offset: u64) -> io::Result<usize> {
        self.cap_std.write_at(buf, offset)
    }

    #[inline]
    fn read_exact_at(&self, buf: &mut [u8], offset: u64) -> io::Result<()> {
        self.cap_std.read_exact_at(buf, offset)
    }

    #[inline]
    fn write_all_at(&self, buf: &[u8], offset: u64) -> io::Result<()> {
        self.cap_std.write_all_at(buf, offset)
    }
}

#[cfg(windows)]
impl std::os::windows::fs::FileExt for File {
    #[inline]
    fn seek_read(&self, buf: &mut [u8], offset: u64) -> io::Result<usize> {
        self.cap_std.seek_read(buf, offset)
    }

    #[inline]
    fn seek_write(&self, buf: &[u8], offset: u64) -> io::Result<usize> {
        self.cap_std.seek_write(buf, offset)
    }
}

// TODO: Use winx to implement "unix" FileExt api on Windows?

// TODO: impl Debug for File? But don't expose File's path...