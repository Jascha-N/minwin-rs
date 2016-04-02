use std::{cmp, mem};
use std::io::{self, Read, Write};
use std::os::raw::c_int;
use winapi as w;

use object::{Readable, Writable};

pub fn check<R, P: FnOnce(&R) -> bool>(result: R, predicate: P) -> io::Result<R> {
    if predicate(&result) {
        Ok(result)
    } else {
        Err(io::Error::last_os_error())
    }
}

pub fn check_bool(result: w::BOOL) -> io::Result<()> {
    check(result, |&result| result != w::FALSE).map(|_| ())
}

pub fn check_pointer<T>(result: *mut T) -> io::Result<*mut T> {
    check(result, |result| !result.is_null())
}

pub fn check_handle(result: w::HANDLE) -> io::Result<w::HANDLE> {
    check_pointer(result)
}

// pub fn check_file_handle(result: w::HANDLE) -> io::Result<w::HANDLE> {
//     check(result, |&result| result != w::INVALID_HANDLE_VALUE)
// }

pub fn check_int(result: c_int) -> io::Result<c_int> {
    check(result, |&result| result != 0)
}

pub fn buffer_size_dword(buffer: &[u8]) -> w::DWORD {
    cmp::min(mem::size_of_val(&buffer[..]),
             w::DWORD::max_value() as usize) as w::DWORD
}

pub struct IoHelper<'a, T: 'a>(pub &'a mut T);

impl<'a, T: Readable> Read for IoHelper<'a, T> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.0.read(buf).map(|read| read as usize)
    }
}

impl<'a, T: Writable> Write for IoHelper<'a, T> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0.write(buf).map(|written| written as usize)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.0.flush()
    }
}
