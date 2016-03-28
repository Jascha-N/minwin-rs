use kernel32 as k32;
use std::error::Error;
use std::ffi::OsStr;
use std::fmt::{self, Display, Formatter};
use std::io::{self, ErrorKind};
use std::os::raw::c_int;
use std::os::windows::ffi::OsStrExt;
use std::ptr;
use winapi as w;

use constants as c;
use util::*;



#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NulError(usize, Vec<u16>);

impl NulError {
    pub fn nul_position(&self) -> usize {
        self.0
    }

    pub fn into_vec(self) -> Vec<u16> {
        self.1
    }
}

impl Error for NulError {
    fn description(&self) -> &str {
        "nul byte found in wide or ANSI string"
    }
}

impl Display for NulError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(formatter,
               "Nul byte found in wide or ANSI string at position: {}",
               self.0)
    }
}

impl From<NulError> for io::Error {
    fn from(error: NulError) -> io::Error {
        io::Error::new(ErrorKind::InvalidInput, error)
    }
}



pub type WideString = Vec<w::WCHAR>;
pub type WideStr = [w::WCHAR];

pub trait ToWideString {
    fn to_wide_string(&self) -> WideString;

    fn to_wide_string_null(&self) -> Result<WideString, NulError> {
        let mut wide = self.to_wide_string();
        if let Some(position) = wide.iter().position(|c| *c == 0) {
            return Err(NulError(position, wide));
        }
        wide.push(0);
        Ok(wide)
    }
}

impl<T: ?Sized> ToWideString for T
    where T: AsRef<OsStr>
{
    fn to_wide_string(&self) -> WideString {
        self.as_ref().encode_wide().collect::<Vec<_>>()
    }
}



pub type AnsiString = Vec<w::CHAR>;
pub type AnsiStr = [w::CHAR];

pub trait ToAnsiString {
    fn to_ansi_string(&self) -> io::Result<AnsiString>;
    fn to_ansi_string_null(&self) -> io::Result<AnsiString>;
}

impl<T: ?Sized> ToAnsiString for T
    where T: ToWideString
{
    fn to_ansi_string(&self) -> io::Result<AnsiString> {
        wide_to_ansi(self.to_wide_string())
    }

    fn to_ansi_string_null(&self) -> io::Result<AnsiString> {
        let result = try!(wide_to_ansi_null(try!(self.to_wide_string_null())));
        Ok(result)
    }
}



fn wide_to_ansi_inner(wide: &WideStr, null_terminated: bool) -> io::Result<AnsiString> {
    let wide_len = if null_terminated {
        -1
    } else {
        wide.len() as c_int
    };
    unsafe {
        let ansi_len = try!(check_int(k32::WideCharToMultiByte(w::CP_ACP,
                                                               c::WC_ERR_INVALID_CHARS,
                                                               wide.as_ptr(),
                                                               wide_len,
                                                               ptr::null_mut(),
                                                               0,
                                                               ptr::null(),
                                                               ptr::null_mut())));

        let mut result = Vec::with_capacity(ansi_len as usize);
        result.set_len(ansi_len as usize);

        let ansi_len = try!(check_int(k32::WideCharToMultiByte(w::CP_ACP,
                                                               c::WC_ERR_INVALID_CHARS,
                                                               wide.as_ptr(),
                                                               wide_len,
                                                               result.as_mut_ptr(),
                                                               ansi_len,
                                                               ptr::null(),
                                                               ptr::null_mut())));

        result.set_len(ansi_len as usize);
        Ok(result)
    }
}

pub fn wide_to_ansi<S: AsRef<WideStr>>(wide: S) -> io::Result<AnsiString> {
    wide_to_ansi_inner(wide.as_ref(), false)
}

pub fn wide_to_ansi_null<S: AsRef<WideStr>>(wide: S) -> io::Result<AnsiString> {
    wide_to_ansi_inner(wide.as_ref(), true)
}



fn ansi_to_wide_inner(ansi: &AnsiStr, null_terminated: bool) -> io::Result<WideString> {
    let ansi_len = if null_terminated {
        -1
    } else {
        ansi.len() as c_int
    };
    unsafe {
        let wide_len = try!(check_int(k32::MultiByteToWideChar(w::CP_ACP,
                                                               c::WC_ERR_INVALID_CHARS,
                                                               ansi.as_ptr(),
                                                               ansi_len,
                                                               ptr::null_mut(),
                                                               0)));

        let mut result = Vec::with_capacity(wide_len as usize);
        result.set_len(wide_len as usize);

        let wide_len = try!(check_int(k32::MultiByteToWideChar(w::CP_ACP,
                                                               c::WC_ERR_INVALID_CHARS,
                                                               ansi.as_ptr(),
                                                               ansi_len,
                                                               result.as_mut_ptr(),
                                                               wide_len)));

        result.set_len(wide_len as usize);
        Ok(result)
    }
}

pub fn ansi_to_wide<S: AsRef<AnsiStr>>(ansi: S) -> io::Result<WideString> {
    ansi_to_wide_inner(ansi.as_ref(), false)
}

pub fn ansi_to_wide_null<S: AsRef<AnsiStr>>(ansi: S) -> io::Result<WideString> {
    ansi_to_wide_inner(ansi.as_ref(), true)
}
