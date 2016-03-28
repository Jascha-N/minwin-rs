use kernel32 as k32;
use std::{io, ptr};
use std::ffi::OsStr;
use std::os::windows::io::{AsRawHandle, FromRawHandle};
use winapi as w;

use access::Access;
use constants as c;
use named::{CreateNamedResult, NamedBuilder, NamedObject, NamedOpenFunction};
use string::WideString;
use util::*;
use waitable::Waitable;



pub struct EventBuilder {
    security_attributes: Option<w::SECURITY_ATTRIBUTES>,
    manual_reset: bool,
    initial_state: bool,
    desired_access: Option<u32>,
}

impl EventBuilder {
    #[cfg_attr(feature = "clippy", allow(new_without_default))]
    pub fn new() -> EventBuilder {
        EventBuilder {
            security_attributes: None,
            manual_reset: false,
            initial_state: false,
            desired_access: None,
        }
    }

    pub fn initial_state(&mut self, signaled: bool) -> &mut EventBuilder {
        self.initial_state = signaled;
        self
    }

    pub fn manual_reset(&mut self, manual: bool) -> &mut EventBuilder {
        self.manual_reset = manual;
        self
    }

    pub fn desired_access<A: Access>(&mut self, desired_access: A) -> &mut EventBuilder {
        self.desired_access = Some(desired_access.mask());
        self
    }
}

impl NamedBuilder for EventBuilder {
    type Output = Event;

    fn __create_inner(&self, name: Option<WideString>) -> io::Result<(Event, bool)> {
        let mut sa = self.security_attributes;
        let sa_ptr = sa.as_mut().map_or(ptr::null_mut(), |sa| sa);
        let name_ptr = name.as_ref().map_or(ptr::null(), |name| name.as_ptr());

        unsafe {
            let handle = match self.desired_access {
                Some(access) => {
                    let mut flags = 0;
                    if self.manual_reset {
                        flags |= c::CREATE_EVENT_MANUAL_RESET
                    }
                    if self.initial_state {
                        flags |= c::CREATE_EVENT_INITIAL_SET
                    }
                    // FIXME: wrong function signature in winapi-rs
                    // See https://github.com/retep998/winapi-rs/pull/270
                    try!(check_handle(k32::CreateEventExW(sa_ptr,
                                                          name_ptr as *mut _,
                                                          flags,
                                                          access)))
                }
                None => {
                    let manual_reset = if self.manual_reset {
                        w::TRUE
                    } else {
                        w::FALSE
                    };
                    let initial_state = if self.initial_state {
                        w::TRUE
                    } else {
                        w::FALSE
                    };
                    try!(check_handle(k32::CreateEventW(sa_ptr,
                                                        manual_reset,
                                                        initial_state,
                                                        name_ptr)))
                }
            };
            let created = k32::GetLastError() != w::ERROR_ALREADY_EXISTS;

            Ok((Event::from_raw_handle(handle), created))
        }
    }
}



object!(Event);

access! { EventAccess,
    ModifyState => c::EVENT_MODIFY_STATE;

    all => c::EVENT_ALL_ACCESS
}

impl Event {
    pub fn create() -> io::Result<Event> {
        EventBuilder::new().create()
    }

    pub fn create_named<N: AsRef<OsStr>>(name: N) -> CreateNamedResult<Event> {
        EventBuilder::new().create_named(name)
    }

    pub fn set(&self) -> io::Result<()> {
        unsafe {
            check_bool(k32::SetEvent(self.as_raw_handle()))
        }
    }

    pub fn reset(&self) -> io::Result<()> {
        unsafe {
            check_bool(k32::ResetEvent(self.as_raw_handle()))
        }
    }
}

impl NamedObject for Event {
    fn __open_function() -> NamedOpenFunction {
        k32::OpenEventW
    }
}

impl Waitable for Event {}
