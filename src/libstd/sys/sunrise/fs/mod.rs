use crate::ffi::OsString;
use crate::fmt;
use crate::hash::{Hash, Hasher};
use crate::io::{self, SeekFrom, IoSlice, IoSliceMut};
use crate::sys::time::SystemTime;
use crate::sys::{unsupported, Void};
use crate::path::{Component, Path, PathBuf};

use crate::sync::Arc;
use crate::collections::HashMap;
use lazy_static::lazy_static;
use sunrise_libuser::fs::{DirectoryEntry, DirectoryEntryType, IFileSystemServiceProxy, IFileSystemProxy, IFileProxy};

use crate::sys::os::getcwd;
use crate::sync::Mutex;

use crate::io::{Error, ErrorKind};

use sunrise_libuser::error::{Error as LibUserError, FileSystemError};

#[stable(feature = "rust1", since = "1.0.0")]
impl From<LibUserError> for Error {
    fn from(user_error: LibUserError) -> Error {
        match user_error {
            LibUserError::FileSystem(error, _) => {
                match error {
                    FileSystemError::Unknown => Error::new(ErrorKind::Other, "Unknown FileSystem IO Error."),
                    FileSystemError::PathNotFound | FileSystemError::FileNotFound | FileSystemError::DirectoryNotFound =>
                        Error::new(ErrorKind::NotFound, "The given resource couldn't be found."),
                    FileSystemError::PathExists => Error::new(ErrorKind::AlreadyExists, "A resource at the given path already exist."),
                    FileSystemError::InUse => Error::new(ErrorKind::Other, "Resource already in use."),
                    FileSystemError::NoSpaceLeft => Error::new(ErrorKind::Other, "There isn't enough space for a resource to be stored."),
                    FileSystemError::InvalidPartition => Error::new(ErrorKind::Other, "The partition wasn't used as it's invalid."),
                    FileSystemError::OutOfRange => Error::new(ErrorKind::Other, "Specified value is out of range."),
                    FileSystemError::WriteFailed => Error::new(ErrorKind::Other, "A write operation failed on the attached storage device."),
                    FileSystemError::ReadFailed => Error::new(ErrorKind::Other, "A read operation failed on the attached storage device."),
                    FileSystemError::PartitionNotFound => Error::new(ErrorKind::Other, "The given partition cannot be found."),
                    FileSystemError::InvalidInput => Error::new(ErrorKind::InvalidInput, "A parameter was incorrect."),
                    FileSystemError::PathTooLong => Error::new(ErrorKind::InvalidData, "The given path is too long to be resolved."),
                    FileSystemError::AccessDenied => Error::new(ErrorKind::PermissionDenied, "The operation lacked the necessary privileges to complete."),
                    FileSystemError::UnsupportedOperation => Error::new(ErrorKind::Other, "The requested operation isn't supported by the detail."),
                    FileSystemError::NotAFile => Error::new(ErrorKind::Other, "The given resource cannot be represented as a file."),
                    FileSystemError::NotADirectory => Error::new(ErrorKind::Other, "The given resource cannot be represented as a directory."),
                    FileSystemError::DiskNotFound => Error::new(ErrorKind::Other, "The given disk id doesn't correspond to a any known disk."),
                    _ => Error::new(ErrorKind::Other, "Unknown Libuser Filesystem Error.")
                }
            },
            _ => Error::new(ErrorKind::Other, "Unknown Libuser IO Error.")
        }
    }
}


lazy_static! {
    /// Registry of all filesystem prefix registered
    static ref SCHEMA_REGISTRY: Mutex<HashMap<&'static str, Arc<IFileSystemProxy>>> = Mutex::new(HashMap::new());
}

#[cfg(not(test))]
pub fn init() {
    let fs_proxy = IFileSystemServiceProxy::raw_new().unwrap();
    let system_filesystem = fs_proxy.open_disk_partition(0, 0).unwrap();
    SCHEMA_REGISTRY.lock().unwrap().insert("system", Arc::new(system_filesystem));
}

fn get_filesystem(path: &Path) -> io::Result<(Arc<IFileSystemProxy>, &str, &Path)> {
    assert!(path.is_absolute(), "path is not absolute?");

    let mut iter = path.components();
    let prefix = match iter.next() {
        Some(Component::Prefix(prefix)) => prefix.as_os_str().to_str().unwrap().trim_end_matches(':'),
        _ => panic!("If path is absolute, it should start with prefix")
    };
    
    for (key, value) in SCHEMA_REGISTRY.lock().unwrap().iter() {
        if prefix == *key {
            return Ok((Arc::clone(&value), prefix, &iter.as_path()))
        }
    }

    unsupported()
}

pub struct File {
    inner: IFileProxy,
    offset: Mutex<u64>
}

pub struct FileAttr(Void);

pub struct ReadDir(Void);

pub struct DirEntry(DirectoryEntry, &'static str);

#[derive(Clone, Debug)]
pub struct OpenOptions {
    read: bool,
    write: bool,
    append: bool,
    truncate: bool,
    create: bool,
    create_new: bool
}

pub struct FilePermissions(Void);

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct FileType(bool);

#[derive(Debug)]
pub struct DirBuilder { }

impl FileAttr {
    pub fn size(&self) -> u64 {
        match self.0 {}
    }

    pub fn perm(&self) -> FilePermissions {
        match self.0 {}
    }

    pub fn file_type(&self) -> FileType {
        match self.0 {}
    }

    pub fn modified(&self) -> io::Result<SystemTime> {
        match self.0 {}
    }

    pub fn accessed(&self) -> io::Result<SystemTime> {
        match self.0 {}
    }

    pub fn created(&self) -> io::Result<SystemTime> {
        match self.0 {}
    }
}

impl Clone for FileAttr {
    fn clone(&self) -> FileAttr {
        match self.0 {}
    }
}

impl FilePermissions {
    pub fn readonly(&self) -> bool {
        match self.0 {}
    }

    pub fn set_readonly(&mut self, _readonly: bool) {
        match self.0 {}
    }
}

impl Clone for FilePermissions {
    fn clone(&self) -> FilePermissions {
        match self.0 {}
    }
}

impl PartialEq for FilePermissions {
    fn eq(&self, _other: &FilePermissions) -> bool {
        match self.0 {}
    }
}

impl Eq for FilePermissions {
}

impl fmt::Debug for FilePermissions {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {}
    }
}

impl FileType {
    pub fn is_dir(&self) -> bool {
        self.0
    }

    pub fn is_file(&self) -> bool {
        !self.is_dir()
    }

    pub fn is_symlink(&self) -> bool {
        false
    }
}

impl fmt::Debug for ReadDir {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {}
    }
}

impl Iterator for ReadDir {
    type Item = io::Result<DirEntry>;

    fn next(&mut self) -> Option<io::Result<DirEntry>> {
        match self.0 {}
    }
}

impl DirEntry {
    pub fn path(&self) -> PathBuf {
        let s = crate::str::from_utf8(&self.0.path).expect("Invalid path for DirEntry");
        let mut res = PathBuf::from(self.1);
        res.push(s);

        res
    }

    pub fn file_name(&self) -> OsString {
        OsString::from(self.path().file_name().expect("No file_name availaible for the DirEntry path"))
    }

    pub fn metadata(&self) -> io::Result<FileAttr> {
        unsupported()
    }

    pub fn file_type(&self) -> io::Result<FileType> {
        Ok(FileType(self.0.directory_entry_type == DirectoryEntryType::Directory))
    }
}

impl OpenOptions {
    pub fn new() -> OpenOptions {
        OpenOptions {
            read: false,
            write: false,
            append: false,
            truncate: false,
            create: false,
            create_new: false
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
        self.append(true);
    }
    pub fn create_new(&mut self, create_new: bool) {
        self.create_new = create_new;
        self.append(true);
    }
}

impl File {
    pub fn open(p: &Path, opts: &OpenOptions) -> io::Result<File> {
        let path = getcwd()?.join(p);
        let (fs, _, path) = get_filesystem(&path)?;

        let path_bytes = path.to_str().unwrap().as_bytes();
        let mut raw_path = [0x0; 0x300];
        raw_path[..path_bytes.len()].copy_from_slice(path_bytes);

        let need_create = opts.create_new || opts.create;

        if need_create {
            let res = fs.create_file(0, 0, &raw_path);

            if res.is_err() && opts.create_new {
                let _ = res?;
            }
        }
        
        let mut flags = 0;

        if opts.read {
            flags |= 1;
        }

        if opts.write {
            flags |= 1 << 1;
        }

        if opts.append {
            flags |= 1 << 2;
        }

        Ok(File {
            inner: fs.open_file(flags, &raw_path)?,
            offset: Mutex::new(0)
        })
    }

    pub fn file_attr(&self) -> io::Result<FileAttr> {
        unsupported()
    }

    pub fn fsync(&self) -> io::Result<()> {
        unsupported()
    }

    pub fn datasync(&self) -> io::Result<()> {
        unsupported()
    }

    pub fn truncate(&self, _size: u64) -> io::Result<()> {
        unsupported()
    }

    pub fn read(&self, buf: &mut [u8]) -> io::Result<usize> {
        let mut buf = buf;
        let mut offset = self.offset.try_lock().unwrap();

        let out = self.inner.read(0, *offset, buf.len() as u64, buf)?;

        *offset += out as u64;

        Ok(out as usize)
    }

    pub fn read_vectored(&self, bufs: &mut [IoSliceMut<'_>]) -> io::Result<usize> {
        crate::io::default_read_vectored(|buf| self.read(buf), bufs)
    }

    pub fn write(&self, buf: &[u8]) -> io::Result<usize> {
        let mut buf = buf;
        let mut offset = self.offset.try_lock().unwrap();

        self.inner.write(0, *offset, buf.len() as u64, buf)?;

        *offset += buf.len() as u64;

        Ok(buf.len())
    }

    pub fn write_vectored(&self, bufs: &[IoSlice<'_>]) -> io::Result<usize> {
        crate::io::default_write_vectored(|buf| self.write(buf), bufs)
    }

    pub fn flush(&self) -> io::Result<()> {
        self.inner.flush()?;

        Ok(())
    }

    pub fn seek(&self, pos: SeekFrom) -> io::Result<u64> {
        let mut offset = self.offset.try_lock().unwrap();

        let newpos = match pos {
            SeekFrom::Current(pos) => {
                let newval = *offset as i64 + pos;

                if newval < 0 {
                    return Err(io::Error::from(io::ErrorKind::InvalidInput));
                } else {
                    *offset = newval as u64;
                }

                newval as u64
            }
            SeekFrom::Start(pos) => {
                *offset = pos;

                pos
            },
            SeekFrom::End(pos) => {
                let size = self.inner.get_size()?;

                let newpos = size as i64 + pos;

                if newpos < 0 {
                    Err(io::Error::from(io::ErrorKind::InvalidInput))?
                }

                *offset = newpos as u64;

                newpos as u64
            }
        };

        Ok(newpos)
    }

    pub fn duplicate(&self) -> io::Result<File> {
        // TODO(Sunrise): Used by try_clone()
        // BODY: To support this we need to make the underlying FileProxy an Arc.
        unsupported()
    }

    pub fn set_permissions(&self, _perm: FilePermissions) -> io::Result<()> {
        // TODO(Sunrise): We don't have permissions at the FS level, what do we do here?
        // BODY: For now we NOP this but should we emulate permissions here?
        Ok(())
    }
}

impl DirBuilder {
    pub fn new() -> DirBuilder {
        DirBuilder { }
    }

    pub fn mkdir(&self, _p: &Path) -> io::Result<()> {
        unsupported()
    }
}

impl fmt::Debug for File {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unimplemented!();
    }
}

pub fn readdir(_p: &Path) -> io::Result<ReadDir> {
    unsupported()
}

pub fn unlink(path: &Path) -> io::Result<()> {
    let path = getcwd()?.join(path);
    let (fs, _, path) = get_filesystem(&path)?;

    let path_bytes = path.to_str().unwrap().as_bytes();

    let mut path = [0x0; 0x300];
    path[..path_bytes.len()].copy_from_slice(path_bytes);

    fs.delete_file(&path)?;

    Ok(())
}

pub fn rename(old: &Path, new: &Path) -> io::Result<()> {
    let old = getcwd()?.join(old);
    let (old_fs, old_prefix, old) = get_filesystem(&old)?;

    let old_path_bytes = old.to_str().unwrap().as_bytes();
    let mut old_path = [0x0; 0x300];
    old_path[..old_path_bytes.len()].copy_from_slice(old_path_bytes);

    let new = getcwd()?.join(new);
    let (new_fs, new_prefix, _) = get_filesystem(&new)?;

    let new_path_bytes = new.to_str().unwrap().as_bytes();
    let mut new_path = [0x0; 0x300];
    new_path[..new_path_bytes.len()].copy_from_slice(new_path_bytes);

    let is_dir = old.is_dir();

    if *old_prefix != *new_prefix {
        return Err(Error::new(ErrorKind::InvalidInput, "Not in the same filesystem"))
    }

    if is_dir {
        old_fs.rename_directory(&old_path, &new_path)?;
    } else {
        old_fs.rename_file(&old_path, &new_path)?;
    }

    Ok(())
}

pub fn set_perm(p: &Path, perm: FilePermissions) -> io::Result<()> {
    let mut opts = OpenOptions::new();

    opts.read(true);
    opts.write(true);

    let file = File::open(p, &opts)?;
    file.set_permissions(perm)
}

pub fn rmdir(path: &Path) -> io::Result<()> {
    let path = getcwd()?.join(path);
    let (fs, _, path) = get_filesystem(&path)?;

    let path_bytes = path.to_str().unwrap().as_bytes();

    let mut path = [0x0; 0x300];
    path[..path_bytes.len()].copy_from_slice(path_bytes);

    fs.delete_directory(&path)?;

    Ok(())
}

pub fn remove_dir_all(_path: &Path) -> io::Result<()> {
    // FIXME: this needs readdir first.
    unsupported()
}

pub fn readlink(_p: &Path) -> io::Result<PathBuf> {
    // FIXME: found the error used for non symlink here.
    unsupported()
}

pub fn symlink(_src: &Path, _dst: &Path) -> io::Result<()> {
    // TODO(Sunrise): We don't have symlink support
    unsupported()
}

pub fn link(_src: &Path, _dst: &Path) -> io::Result<()> {
    unsupported()
}

pub fn stat(_p: &Path) -> io::Result<FileAttr> {
    unsupported()
}

pub fn lstat(_p: &Path) -> io::Result<FileAttr> {
    unsupported()
}

pub fn canonicalize(_p: &Path) -> io::Result<PathBuf> {
    unsupported()
}

pub fn copy(from: &Path, to: &Path) -> io::Result<u64> {
    use crate::fs::File;

    if !from.is_file() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput,
                              "the source path is not an existing regular file"))
    }

    let mut reader = File::open(from)?;
    let mut writer = File::create(to)?;
    let perm = reader.metadata()?.permissions();

    let ret = io::copy(&mut reader, &mut writer)?;
    writer.set_permissions(perm)?;
    Ok(ret)
}
