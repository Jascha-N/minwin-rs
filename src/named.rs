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
            CreateNamedError::InvalidName(ref error) => error.description(),
            CreateNamedError::Io(ref error) => error.description()
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            CreateNamedError::InvalidName(ref error) => Some(error),
            CreateNamedError::Io(ref error) => Some(error),
            _ => None
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



/// A trait containing common methods for creating named objects.
pub trait NamedBuilder {
    type Output: NamedObject;

    #[doc(hidden)]
    fn __create_inner(&self, name: Option<WideString>) -> io::Result<(Self::Output, bool)>;

    /// Creates a new anonymous object.
    fn create(&self) -> io::Result<Self::Output> {
        let (result, _) = try!(self.__create_inner(None));
        Ok(result)
    }

    /// Creates a new named object or opens an existing object.
    fn create_named<N: AsRef<OsStr>>(&self, name: N) -> CreateNamedResult<Self::Output> {
        let (result, created) = try!(self.__create_inner(Some(try!(name.to_wide_string_null()))));
        if created {
            Ok(result)
        } else {
            Err(CreateNamedError::AlreadyExists(result))
        }
    }
}



/// Windows API function type used for opening named objects.
///
/// Used internally by `NamedObject`s.
pub type NamedOpenFunction = unsafe extern "system" fn(w::DWORD, w::BOOL, w::LPCWSTR) -> w::HANDLE;



/// A type that can be constructed given an existing object name.
pub trait NamedObject: ObjectExt {
    /// The extern function used for opening an object of this type.
    fn open_function() -> NamedOpenFunction;

    /// The default opening options for this type.
    fn default_open_options() -> NamedOpenOptions;

    /// Opens a named object using default options.
    fn open<N: AsRef<OsStr>>(name: N) -> io::Result<Self> {
        Self::open_with_options(name, Self::default_open_options())
    }

    /// Opens a named object using the specified options.
    fn open_with_options<N, O>(name: N, options: O) -> io::Result<Self>
        where N: AsRef<OsStr>,
              O: Into<NamedOpenOptions>
    {
        let name = try!(name.to_wide_string_null());
        let options = options.into();
        let inherit = if options.inheritable {
            w::TRUE
        } else {
            w::FALSE
        };

        unsafe {
            let handle = try!(check_handle(Self::open_function()(options.desired_access,
                                                                 inherit,
                                                                 name.as_ptr())));
            Ok(Self::from_raw_handle(handle))
        }
    }
}



/// Options for opening a named object.
#[derive(Debug)]
pub struct NamedOpenOptions {
    inheritable: bool,
    desired_access: u32
}

impl NamedOpenOptions {
    pub fn new() -> NamedOpenOptions {
        NamedOpenOptions {
            inheritable: false,
            desired_access: MaximumAccess.mask()
        }
    }

    /// Indicates whether the underlying handle can be inherited.
    ///
    /// If the parameter is `true`, processes created by this process will inherit the handle.
    /// Otherwise, the processes do not inherit this handle.
    pub fn inheritable(mut self, inheritable: bool) -> Self {
        self.inheritable = inheritable;
        self
    }

    /// The desired access for the object.
    pub fn desired_access<A: Access>(mut self, desired_access: A) -> Self {
        self.desired_access = desired_access.mask();
        self
    }
}

impl<A: Access> From<A> for NamedOpenOptions {
    fn from(access: A) -> NamedOpenOptions {
        NamedOpenOptions::new().desired_access(access)
    }
}
