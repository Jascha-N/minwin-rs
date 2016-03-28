use kernel32 as k32;
use chrono::Duration;
use std::{io, mem, ptr};
use std::ffi::OsStr;
use std::os::windows::io::{AsRawHandle, FromRawHandle};
use winapi as w;

use access::Access;
use constants as c;
use named::{CreateNamedResult, NamedBuilder, NamedObject, NamedOpenFunction};
use string::WideString;
use util::*;
use waitable::{Waitable, WaitError};



pub struct SemaphoreBuilder {
    security_attributes: Option<w::SECURITY_ATTRIBUTES>,
    initial_count: i32,
    maximum_count: i32,
    desired_access: Option<u32>,
}

impl SemaphoreBuilder {
    pub fn new(maximum_count: i32) -> SemaphoreBuilder {
        SemaphoreBuilder {
            security_attributes: None,
            initial_count: maximum_count,
            maximum_count: maximum_count,
            desired_access: None,
        }
    }

    pub fn initial_count(&mut self, count: i32) -> &mut SemaphoreBuilder {
        self.initial_count = count;
        self
    }

    pub fn desired_access<A: Access>(&mut self, desired_access: A) -> &mut SemaphoreBuilder {
        self.desired_access = Some(desired_access.mask());
        self
    }
}

impl NamedBuilder for SemaphoreBuilder {
    type Output = Semaphore;

    fn __create_inner(&self, name: Option<WideString>) -> io::Result<(Semaphore, bool)> {
        let mut sa = self.security_attributes;
        let sa_ptr = sa.as_mut().map_or(ptr::null_mut(), |sa| sa);
        let name_ptr = name.as_ref().map_or(ptr::null(), |name| name.as_ptr());

        unsafe {
            let handle = match self.desired_access {
                Some(access) => {
                    try!(check_handle(k32::CreateSemaphoreExW(sa_ptr,
                                                              self.initial_count,
                                                              self.maximum_count,
                                                              name_ptr,
                                                              0,
                                                              access)))
                }
                None => {
                    try!(check_handle(k32::CreateSemaphoreW(sa_ptr,
                                                            self.initial_count,
                                                            self.maximum_count,
                                                            name_ptr)))
                }
            };
            let created = k32::GetLastError() != w::ERROR_ALREADY_EXISTS;

            Ok((Semaphore::from_raw_handle(handle), created))
        }
    }
}



object!(Semaphore);

access! { SemaphoreAccess,
    ModifyState => c::SEMAPHORE_MODIFY_STATE;

    all => c::SEMAPHORE_ALL_ACCESS
}

impl Semaphore {
    pub fn create(maximum_count: i32) -> io::Result<Semaphore> {
        SemaphoreBuilder::new(maximum_count).create()
    }

    pub fn create_named<N: AsRef<OsStr>>(name: N,
                                         maximum_count: i32)
                                         -> CreateNamedResult<Semaphore> {
        SemaphoreBuilder::new(maximum_count).create_named(name)
    }

    pub fn obtain(&self) -> io::Result<SemaphoreGuard> {
        match self.wait() {
            Ok(()) => Ok(self.guard(1)),
            Err(WaitError::Io(error)) => Err(error),
            _ => unreachable!(),
        }
    }

    pub fn try_obtain(&self) -> Result<SemaphoreGuard, TryObtainError> {
        match self.wait_timeout(Duration::zero()) {
            Ok(()) => Ok(self.guard(1)),
            Err(WaitError::Timeout) => Err(TryObtainError::WouldBlock),
            Err(WaitError::Io(error)) => Err(TryObtainError::Io(error)),
            _ => unreachable!(),
        }
    }

    pub fn release(&self, count: i32) -> io::Result<i32> {
        unsafe {
            let mut previous_count = mem::uninitialized();
            try!(check_bool(k32::ReleaseSemaphore(self.as_raw_handle(),
                                                  count,
                                                  &mut previous_count)));
            Ok(previous_count)
        }
    }

    pub fn guard(&self, release_count: i32) -> SemaphoreGuard {
        SemaphoreGuard {
            semaphore: self,
            release_count: release_count,
        }
    }
}

impl NamedObject for Semaphore {
    fn __open_function() -> NamedOpenFunction {
        k32::OpenSemaphoreW
    }
}

impl Waitable for Semaphore {}



#[derive(Debug)]
pub struct SemaphoreGuard<'a> {
    semaphore: &'a Semaphore,
    release_count: i32,
}

impl<'a> Drop for SemaphoreGuard<'a> {
    fn drop(&mut self) {
        debug_assert!(self.semaphore.release(self.release_count).is_ok());
    }
}

#[derive(Debug)]
pub enum TryObtainError {
    WouldBlock,
    Io(io::Error),
}
