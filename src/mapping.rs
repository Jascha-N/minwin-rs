use kernel32 as k32;
use std::{io, mem, ptr, slice};
use std::ffi::OsStr;
use std::os::windows::io::{AsRawHandle, FromRawHandle};
use winapi as w;

use handle::Handle;
use named::{CreateNamedResult, NamedBuilder, NamedObject, NamedOpenFunction, NamedOpenOptions};
use string::WideString;
use util::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum FileMappingAttribute {
    Commit,
    Image,
    ImageNoExecute,
    LargePages,
    NoCache,
    Reserve,
    WriteCombine,
}

pub struct FileMappingBuilder<'a> {
    file: Option<&'a AsRawHandle>,
    security_attributes: Option<w::SECURITY_ATTRIBUTES>,
    writable: bool,
    executable: bool,
    attributes: u32,
    size: u64,
    name: Option<&'a OsStr>,
}

impl<'a> FileMappingBuilder<'a> {
    pub fn new(size: u64) -> FileMappingBuilder<'a> {
        FileMappingBuilder {
            file: None,
            security_attributes: None,
            writable: true,
            executable: false,
            attributes: 0,
            size: size,
            name: None,
        }
    }

    pub fn from_file<F: AsRawHandle>(file: &'a F) -> FileMappingBuilder<'a> {
        FileMappingBuilder {
            file: Some(file),
            security_attributes: None,
            writable: true,
            executable: false,
            attributes: 0,
            size: 0,
            name: None,
        }
    }

    pub fn size(&mut self, size: u64) -> &mut FileMappingBuilder<'a> {
        self.size = size;
        self
    }

    pub fn writable(&mut self, writable: bool) -> &mut FileMappingBuilder<'a> {
        self.writable = writable;
        self
    }

    pub fn executable(&mut self, executable: bool) -> &mut FileMappingBuilder<'a> {
        self.executable = executable;
        self
    }

    pub fn attribute(&mut self, attribute: FileMappingAttribute) -> &mut FileMappingBuilder<'a> {
        self.attributes |= match attribute {
            FileMappingAttribute::Commit => w::SEC_COMMIT,
            FileMappingAttribute::Image => w::SEC_IMAGE,
            FileMappingAttribute::ImageNoExecute => w::SEC_IMAGE_NO_EXECUTE,
            FileMappingAttribute::LargePages => w::SEC_LARGE_PAGES,
            FileMappingAttribute::NoCache => w::SEC_NOCACHE,
            FileMappingAttribute::Reserve => w::SEC_RESERVE,
            FileMappingAttribute::WriteCombine => w::SEC_WRITECOMBINE,
        };
        self
    }

    pub fn attributes(&mut self,
                      attributes: &[FileMappingAttribute])
                      -> &mut FileMappingBuilder<'a> {
        for &attr in attributes {
            self.attribute(attr);
        }
        self
    }

    pub fn name<N: AsRef<OsStr>>(&mut self, name: &'a N) -> &mut FileMappingBuilder<'a> {
        self.name = Some(name.as_ref());
        self
    }
}

impl<'a> NamedBuilder for FileMappingBuilder<'a> {
    type Output = FileMapping;

    #[doc(hidden)]
    fn __create_inner(&self, name: Option<WideString>) -> io::Result<(FileMapping, bool)> {
        let protect = self.attributes |
                      match (self.writable, self.executable) {
            (false, false) => w::PAGE_READONLY,
            (true, false) => w::PAGE_READWRITE,
            (false, true) => w::PAGE_EXECUTE_READ,
            (true, true) => w::PAGE_EXECUTE_READWRITE,
        };

        let mut sa = self.security_attributes;

        unsafe {
            let handle = try!(check_handle(
                k32::CreateFileMappingW(self.file.map_or(w::INVALID_HANDLE_VALUE,
                                                         AsRawHandle::as_raw_handle),
                                        sa.as_mut().map_or(ptr::null_mut(), |sa| sa),
                                        protect,
                                        (self.size >> 32) as w::DWORD,
                                        self.size as w::DWORD,
                                        name.as_ref().map_or(ptr::null(), |name| name.as_ptr()))
            ));
            let created = k32::GetLastError() != w::ERROR_ALREADY_EXISTS;
            let result = FileMapping::from_raw_handle(handle);

            Ok((result, created))
        }
    }
}

#[derive(Debug)]
pub struct FileMapping(Handle);

handle!(FileMapping);

access! { FileMappingAccess,
    Read => w::FILE_MAP_READ,
    Write => w::FILE_MAP_WRITE,
    Execute => w::FILE_MAP_EXECUTE;

    all => w::FILE_MAP_ALL_ACCESS
}

impl FileMapping {
    pub fn create(size: u64) -> io::Result<FileMapping> {
        FileMappingBuilder::new(size).create()
    }

    pub fn create_named<N: AsRef<OsStr>>(name: N, size: u64) -> CreateNamedResult<FileMapping> {
        FileMappingBuilder::new(size).create_named(name)
    }

    pub fn create_from_file<F: AsRawHandle>(file: &F) -> io::Result<FileMapping> {
        FileMappingBuilder::from_file(file).create()
    }

    pub fn full(&self) -> io::Result<FileView> {
        self.view().map()
    }

    pub fn range(&self, offset: u64, size: usize) -> io::Result<FileView> {
        self.view().offset(offset).size(size).map()
    }

    pub fn view(&self) -> FileViewBuilder {
        FileViewBuilder {
            mapping: self,
            offset: 0,
            size: 0,
            write_mode: FileViewWriteMode::ReadOnly,
            executable: false,
        }
    }

    pub fn flush(&self) -> io::Result<()> {
        unsafe { check_bool(k32::FlushFileBuffers(self.as_raw_handle())) }
    }
}

impl NamedObject for FileMapping {
    fn open_function() -> NamedOpenFunction {
        k32::OpenFileMappingW
    }

    fn default_open_options() -> NamedOpenOptions {
        FileMappingAccess::all().into()
    }
}



#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum FileViewWriteMode {
    ReadOnly,
    Write,
    CopyOnWrite,
}

pub struct FileViewBuilder<'a> {
    mapping: &'a FileMapping,
    offset: u64,
    size: usize,
    write_mode: FileViewWriteMode,
    executable: bool,
}

impl<'a> FileViewBuilder<'a> {
    pub fn write_mode(&mut self, write_mode: FileViewWriteMode) -> &mut FileViewBuilder<'a> {
        self.write_mode = write_mode;
        self
    }

    pub fn executable(&mut self, executable: bool) -> &mut FileViewBuilder<'a> {
        self.executable = executable;
        self
    }

    pub fn offset(&mut self, offset: u64) -> &mut FileViewBuilder<'a> {
        self.offset = offset;
        self
    }

    pub fn size(&mut self, size: usize) -> &mut FileViewBuilder<'a> {
        self.size = size;
        self
    }

    pub fn map(&self) -> io::Result<FileView> {
        let mut access = match self.write_mode {
            FileViewWriteMode::ReadOnly => w::FILE_MAP_READ,
            FileViewWriteMode::Write => w::FILE_MAP_WRITE,
            FileViewWriteMode::CopyOnWrite => w::FILE_MAP_COPY,
        };
        if self.executable {
            access |= w::FILE_MAP_EXECUTE
        };
        unsafe {
            let address = try!(check_pointer(k32::MapViewOfFile(self.mapping.as_raw_handle(),
                                                                access,
                                                                (self.offset >> 32) as w::DWORD,
                                                                self.offset as w::DWORD,
                                                                self.size as w::SIZE_T)));

            let mut mem_info = mem::uninitialized();
            try!(check(k32::VirtualQuery(address,
                                        &mut mem_info,
                                        mem::size_of::<w::MEMORY_BASIC_INFORMATION>() as w::SIZE_T),
                       |&result| result != 0));

            Ok(FileView {
                address: address as *mut _,
                size: mem_info.RegionSize as usize,
            })
        }
    }
}

pub struct FileView {
    address: *mut u8,
    size: usize,
}

impl FileView {
    pub fn as_ptr(&self) -> *const u8 {
        self.address
    }

    pub fn as_mut_ptr(&mut self) -> *mut u8 {
        self.address
    }

    pub unsafe fn as_slice(&self) -> &[u8] {
        slice::from_raw_parts(self.address, self.size)
    }

    pub unsafe fn as_mut_slice(&mut self) -> &mut [u8] {
        slice::from_raw_parts_mut(self.address, self.size)
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn flush_range(&self, offset: Option<usize>, size: Option<usize>) -> io::Result<()> {
        unsafe {
            check_bool(k32::FlushViewOfFile(self.address.offset(offset.unwrap_or(0) as isize) as w::LPCVOID,
                                            size.unwrap_or(0) as w::SIZE_T))
        }
    }

    pub fn flush(&self) -> io::Result<()> {
        self.flush_range(None, None)
    }
}

impl Drop for FileView {
    fn drop(&mut self) {
        unsafe {
            debug_assert!(k32::UnmapViewOfFile(self.address as w::LPVOID) != w::FALSE);
        }
    }
}

unsafe impl Send for FileView {}
unsafe impl Sync for FileView {}
