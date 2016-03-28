use std::io;
use std::os::raw::c_int;
use winapi as w;

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
