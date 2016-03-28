use kernel32 as k32;
use std::{cmp, io, mem, ptr};
use std::os::windows::io::{AsRawHandle, FromRawHandle, IntoRawHandle};
use winapi as w;

use handle::{self, Handle};
use overlapped::Overlapped;
use util::*;

pub trait Object: AsRawHandle {
    fn set_inheritable(&self, inherit: bool) -> io::Result<()> {
        handle::set_inheritable(self.as_raw_handle(), inherit)
    }

    fn is_inheritable(&self) -> io::Result<bool> {
        handle::is_inheritable(self.as_raw_handle())
    }

    fn set_protected(&self, protect: bool) -> io::Result<()> {
        handle::set_protected(self.as_raw_handle(), protect)
    }

    fn is_protected(&self) -> io::Result<bool> {
        handle::is_protected(self.as_raw_handle())
    }
}

pub trait ObjectExt: Object + FromRawHandle + IntoRawHandle + Sized {
    fn try_clone(&self) -> io::Result<Self> {
        handle::duplicate(self.as_raw_handle())
               .map(|handle| unsafe { Self::from_raw_handle(handle) })
    }

    unsafe fn from_handle(handle: Handle) -> Self {
        Self::from_raw_handle(handle.into_raw_handle())
    }

    fn into_handle(self) -> Handle {
        unsafe { Handle::from_raw_handle(self.into_raw_handle()) }
    }

    fn close(self) -> io::Result<()> {
        unsafe { check_bool(k32::CloseHandle(self.into_raw_handle())) }
    }
}

impl<T: ?Sized> Object for T where T: AsRawHandle {}
impl<T> ObjectExt for T where T: Object + FromRawHandle + IntoRawHandle {}

pub trait Readable: Object {
    fn read(&self, buffer: &mut [u8]) -> io::Result<u32> {
        unsafe {
            let mut read = mem::uninitialized();
            let len = cmp::min(buffer.len(), <w::DWORD>::max_value() as usize) as w::DWORD;
            try!(check_bool(k32::ReadFile(self.as_raw_handle(), buffer.as_mut_ptr() as *mut _, len,
                                          &mut read, ptr::null_mut())));
            Ok(read)
        }
    }

    unsafe fn read_overlapped(&self, buffer: &mut [u8], overlapped: &mut Overlapped) -> io::Result<bool> {
        let len = cmp::min(buffer.len(), <w::DWORD>::max_value() as usize) as w::DWORD;
        let res = check_bool(k32::ReadFile(self.as_raw_handle(), buffer.as_mut_ptr() as *mut _, len,
                                           ptr::null_mut(), overlapped.get()));
        match res {
            Ok(_) => Ok(true),
            Err(ref e) if e.raw_os_error() == Some(w::ERROR_IO_PENDING as i32) => Ok(false),
            Err(e) => Err(e),
        }
    }
}

pub trait Writable: Object {
    fn write(&self, buffer: &[u8]) -> io::Result<u32> {
        unsafe {
            let mut written = mem::uninitialized();
            let len = cmp::min(buffer.len(), <w::DWORD>::max_value() as usize) as w::DWORD;
            try!(check_bool(k32::WriteFile(self.as_raw_handle(), buffer.as_ptr() as *const _, len,
                                           &mut written, ptr::null_mut())));
            Ok(written)
        }
    }

    unsafe fn write_overlapped(&self, buffer: &mut [u8], overlapped: &mut Overlapped) -> io::Result<bool> {
        let len = cmp::min(buffer.len(), <w::DWORD>::max_value() as usize) as w::DWORD;
        let res = check_bool(k32::WriteFile(self.as_raw_handle(), buffer.as_ptr() as *const _, len,
                                            ptr::null_mut(), overlapped.get()));
        match res {
            Ok(_) => Ok(true),
            Err(ref e) if e.raw_os_error() == Some(w::ERROR_IO_PENDING as i32) => Ok(false),
            Err(e) => Err(e),
        }
    }

    fn flush(&self) -> io::Result<()> {
        unsafe {
            check_bool(k32::FlushFileBuffers(self.as_raw_handle())).map(|_| ())
        }
    }
}
