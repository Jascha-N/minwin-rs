use chrono::Duration;
use kernel32 as k32;
use std::{io, ptr, thread};
use std::error::Error;
use std::ffi::OsStr;
use std::fmt::{self, Display, Formatter};
use std::marker::PhantomData;
use std::os::windows::io::{AsRawHandle, FromRawHandle};
use winapi as w;

use access::Access;
use constants as c;
use handle::Handle;
use named::{CreateNamedResult, NamedBuilder, NamedObject, NamedOpenFunction, NamedOpenOptions};
use string::WideString;
use util::*;
use waitable::{Waitable, WaitError};



pub struct MutexBuilder {
    security_attributes: Option<w::SECURITY_ATTRIBUTES>,
    initial_owner: bool,
    desired_access: Option<u32>,
}

impl MutexBuilder {
    #[cfg_attr(feature = "clippy", allow(new_without_default))]
    pub fn new() -> MutexBuilder {
        MutexBuilder {
            security_attributes: None,
            initial_owner: false,
            desired_access: None,
        }
    }

    // Does not really fit in the current design.
    pub fn initial_owner(&mut self, owned: bool) -> &mut MutexBuilder {
        self.initial_owner = owned;
        self
    }

    pub fn desired_access<A: Access>(&mut self, desired_access: A) -> &mut MutexBuilder {
        self.desired_access = Some(desired_access.mask());
        self
    }
}

impl NamedBuilder for MutexBuilder {
    type Output = Mutex;

    #[doc(hidden)]
    fn __create_inner(&self, name: Option<WideString>) -> io::Result<(Mutex, bool)> {
        let mut sa = self.security_attributes;
        let sa_ptr = sa.as_mut().map_or(ptr::null_mut(), |sa| sa);
        let name_ptr = name.as_ref().map_or(ptr::null(), |name| name.as_ptr());

        unsafe {
            let handle = match self.desired_access {
                Some(access) => {
                    let flags = if self.initial_owner {
                        c::CREATE_MUTEX_INITIAL_OWNER
                    } else {
                        0
                    };
                    try!(check_handle(k32::CreateMutexExW(sa_ptr, name_ptr, flags, access)))
                }
                None => {
                    let initial_owner = if self.initial_owner {
                        w::TRUE
                    } else {
                        w::FALSE
                    };
                    try!(check_handle(k32::CreateMutexW(sa_ptr, initial_owner, name_ptr)))
                }
            };
            let created = k32::GetLastError() != w::ERROR_ALREADY_EXISTS;

            Ok((Mutex::from_raw_handle(handle), created))
        }
    }
}



#[derive(Debug)]
pub struct Mutex(Handle);

handle!(Mutex);

access! { MutexAccess,
    ModifyState => c::MUTEX_MODIFY_STATE;

    all => c::MUTEX_ALL_ACCESS
}

impl Mutex {
    pub fn create() -> io::Result<Mutex> {
        MutexBuilder::new().create()
    }

    pub fn create_named<N: AsRef<OsStr>>(name: N) -> CreateNamedResult<Mutex> {
        MutexBuilder::new().create_named(name)
    }

    pub fn lock(&self) -> Result<MutexGuard, LockError> {
        match self.wait() {
            Ok(_) => Ok(self.guard()),
            Err(WaitError::Abandoned(_)) => Err(LockError::Abandoned(self.guard())),
            Err(WaitError::Io(error)) => Err(LockError::Io(error)),
            _ => unreachable!(),
        }
    }

    pub fn try_lock(&self) -> Result<MutexGuard, TryLockError> {
        match self.wait_timeout(Duration::zero()) {
            Ok(_) => Ok(self.guard()),
            Err(WaitError::Timeout) => Err(TryLockError::WouldBlock),
            Err(WaitError::Abandoned(_)) => Err(TryLockError::Abandoned(self.guard())),
            Err(WaitError::Io(error)) => Err(TryLockError::Io(error)),
        }
    }

    pub fn release(&self) -> io::Result<()> {
        unsafe {
            check_bool(k32::ReleaseMutex(self.as_raw_handle()))
        }
    }

    pub fn guard(&self) -> MutexGuard {
        MutexGuard(self, PhantomData)
    }
}

impl NamedObject for Mutex {
    fn open_function() -> NamedOpenFunction {
        k32::OpenMutexW
    }

    fn default_open_options() -> NamedOpenOptions {
        MutexAccess::all().into()
    }
}

impl Waitable for Mutex {}



#[derive(Debug)]
pub struct MutexGuard<'a>(&'a Mutex, PhantomData<::std::sync::MutexGuard<'a, ()>>);

impl<'a> Drop for MutexGuard<'a> {
    fn drop(&mut self) {
        // Leak the lock on purpose if a panic occurs.
        if !thread::panicking() {
            debug_assert!(self.0.release().is_ok());
        }
    }
}



#[derive(Debug)]
pub enum LockError<'a> {
    Abandoned(MutexGuard<'a>),
    Io(io::Error),
}

impl<'a> Error for LockError<'a> {
    fn description(&self) -> &str {
        match *self {
            LockError::Abandoned(_) => "abandoned mutex",
            LockError::Io(ref error) => error.description()
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            LockError::Io(ref error) => Some(error),
            _ => None
        }
    }
}

impl<'a> Display for LockError<'a> {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match *self {
            LockError::Abandoned(_) => write!(formatter, "The thread that owned the mutex terminated"),
            LockError::Io(ref error) => write!(formatter, "An I/O error occurred: {}", error)
        }
    }
}



#[derive(Debug)]
pub enum TryLockError<'a> {
    Abandoned(MutexGuard<'a>),
    WouldBlock,
    Io(io::Error),
}

impl<'a> Error for TryLockError<'a> {
    fn description(&self) -> &str {
        match *self {
            TryLockError::Abandoned(_) => "abandoned mutex",
            TryLockError::WouldBlock => "operation would block",
            TryLockError::Io(ref error) => error.description()
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            TryLockError::Io(ref error) => Some(error),
            _ => None
        }
    }
}

impl<'a> Display for TryLockError<'a> {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match *self {
            TryLockError::Abandoned(_) => write!(formatter, "The thread that owned the mutex terminated"),
            TryLockError::WouldBlock => write!(formatter, "The operation would block"),
            TryLockError::Io(ref error) => write!(formatter, "An I/O error occurred: {}", error)
        }
    }
}
