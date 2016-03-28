use kernel32 as k32;
use std::{io, mem};
use std::os::windows::io::{AsRawHandle, FromRawHandle, IntoRawHandle, RawHandle};
use winapi as w;

use constants as c;
use util::*;



#[derive(Debug)]
pub struct Handle(RawHandle);

impl Drop for Handle {
    fn drop(&mut self) {
        unsafe {
            debug_assert!(k32::CloseHandle(self.0) != w::FALSE);
        }
    }
}

impl FromRawHandle for Handle {
    unsafe fn from_raw_handle(handle: RawHandle) -> Handle {
        Handle(handle)
    }
}

impl AsRawHandle for Handle {
    fn as_raw_handle(&self) -> RawHandle {
        self.0
    }
}

impl IntoRawHandle for Handle {
    fn into_raw_handle(self) -> RawHandle {
        let result = self.0;
        mem::forget(self);
        result
    }
}

unsafe impl Send for Handle {}
unsafe impl Sync for Handle {}



fn set_flag(handle: RawHandle, flag: u32, enabled: bool) -> io::Result<()> {
    let mask = flag;
    let flags = if enabled {
        flag
    } else {
        0
    };
    unsafe {
        check_bool(k32::SetHandleInformation(handle, mask, flags))
    }
}

fn get_flag(handle: RawHandle, flag: u32) -> io::Result<bool> {
    unsafe {
        let mut flags = mem::uninitialized();
        try!(check_bool(k32::GetHandleInformation(handle, &mut flags)));
        Ok(flags & flag != 0)
    }
}

pub fn duplicate(handle: RawHandle) -> io::Result<RawHandle> {
    unsafe {
        let current_process = k32::GetCurrentProcess();
        let mut cloned_handle = mem::uninitialized();
        try!(check_bool(k32::DuplicateHandle(current_process,
                                             handle,
                                             current_process,
                                             &mut cloned_handle,
                                             0,
                                             w::FALSE,
                                             w::DUPLICATE_SAME_ACCESS)));

        Ok(cloned_handle)
    }
}

pub fn set_inheritable(handle: RawHandle, inherit: bool) -> io::Result<()> {
    set_flag(handle, c::HANDLE_FLAG_INHERIT, inherit)
}

pub fn is_inheritable(handle: RawHandle) -> io::Result<bool> {
    get_flag(handle, c::HANDLE_FLAG_INHERIT)
}

pub fn set_protected(handle: RawHandle, protect: bool) -> io::Result<()> {
    set_flag(handle, c::HANDLE_FLAG_PROTECT_FROM_CLOSE, protect)
}

pub fn is_protected(handle: RawHandle) -> io::Result<bool> {
    get_flag(handle, c::HANDLE_FLAG_PROTECT_FROM_CLOSE)
}
