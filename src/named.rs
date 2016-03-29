use std::io;
use std::error::Error;
use std::fmt::{self, Debug, Display, Formatter};
use std::any::Any;
use std::ffi::OsStr;
use winapi as w;

use access::{Access, MaximumAccess};
use object::ObjectExt;
use string::{ToWideString, WideString, NulError};
use util::*;

#[derive(Debug)]
pub enum CreateNamedError<T> {
    AlreadyExists(T),
    InvalidName(NulError),
    Io(io::Error),
}

impl<T: Debug + Any> Error for CreateNamedError<T> {
    fn description(&self) -> &str {
        match *self {
            CreateNamedError::AlreadyExists(_) => "named object already exists",
            CreateNamedError::InvalidName(_) => "invalid name",
            CreateNamedError::Io(_) => "I/O error"
        }
    }
}

impl<T> Display for CreateNamedError<T> {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match *self {
            CreateNamedError::AlreadyExists(_) => write!(formatter, "Named object already exists"),
            CreateNamedError::InvalidName(ref error) => write!(formatter, "{}", error),
            CreateNamedError::Io(ref error) => write!(formatter, "An I/O error occurred: {}", error)
        }
    }
}

impl<T> From<io::Error> for CreateNamedError<T> {
    fn from(error: io::Error) -> CreateNamedError<T> {
        CreateNamedError::Io(error)
    }
}

impl<T> From<NulError> for CreateNamedError<T> {
    fn from(error: NulError) -> CreateNamedError<T> {
        CreateNamedError::InvalidName(error)
    }
}



pub type CreateNamedResult<T> = Result<T, CreateNamedError<T>>;

impl<T> CreateNamedError<T> {
    pub fn unwrap(self) -> Option<T> {
        match self {
            CreateNamedError::AlreadyExists(object) => Some(object),
            _ => None
        }
    }
}

pub trait NamedBuilder {
    type Output: NamedObject;

    #[doc(hidden)]
    fn __create_inner(&self, name: Option<WideString>) -> io::Result<(Self::Output, bool)>;

    fn create(&self) -> io::Result<Self::Output> {
        let (result, _) = try!(self.__create_inner(None));
        Ok(result)
    }

    fn create_named<N: AsRef<OsStr>>(&self, name: N) -> CreateNamedResult<Self::Output> {
        let (result, created) = try!(self.__create_inner(Some(try!(name.to_wide_string_null()))));
        if created {
            Ok(result)
        } else {
            Err(CreateNamedError::AlreadyExists(result))
        }
    }
}

pub type NamedOpenFunction = unsafe extern "system" fn(w::DWORD, w::BOOL, w::LPCWSTR) -> w::HANDLE;

pub trait NamedObject: ObjectExt {
    #[doc(hidden)]
    fn __open_function() -> NamedOpenFunction;

    fn open_named<N: AsRef<OsStr>>(name: N) -> io::Result<Self> {
        Self::open_named_with_access(name, MaximumAccess)
    }

    fn open_named_with_access<N: AsRef<OsStr>, A: Access>(name: N,
                                                          desired_access: A)
                                                          -> io::Result<Self> {
        let name = try!(name.to_wide_string_null());
        unsafe {
            let handle = try!(check_handle(Self::__open_function()(desired_access.mask(),
                                                                   w::FALSE,
                                                                   name.as_ptr())));
            Ok(Self::from_raw_handle(handle))
        }
    }
}
