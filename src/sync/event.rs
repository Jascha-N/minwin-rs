use kernel32 as k32;
use std::{io, ptr};
use std::ffi::OsStr;
use std::os::windows::io::{AsRawHandle, FromRawHandle};
use winapi as w;

use access::Access;
use constants as c;
use handle::Handle;
use named::{CreateNamedResult, NamedBuilder, NamedObject, NamedOpenFunction, NamedOpenOptions};
use string::WideString;
use util::*;
use waitable::Waitable;



/// A builder for creating a new `Event`.
///
/// See [CreateEvent][CreateEvent] and [CreateEventEx][CreateEventEx] for more
/// information.
///
/// [CreateEvent]: https://msdn.microsoft.com/en-us/library/windows/desktop/ms682396%28v=vs.85%29.aspx
/// [CreateEventEx]: https://msdn.microsoft.com/en-us/library/windows/desktop/ms682400%28v=vs.85%29.aspx
pub struct EventBuilder {
    security_attributes: Option<w::SECURITY_ATTRIBUTES>,
    manual_reset: bool,
    initial_state: bool,
    desired_access: Option<u32>,
}

impl EventBuilder {
    /// Creates a new event builder with the default settings.
    #[cfg_attr(feature = "clippy", allow(new_without_default))]
    pub fn new() -> EventBuilder {
        EventBuilder {
            security_attributes: None,
            manual_reset: false,
            initial_state: false,
            desired_access: None,
        }
    }

    /// The reset mode of the event.
    ///
    /// If the parameter is `true`, the builder creates a manual-reset event object,
    /// which requires the use of the `Event::reset()` function to set the event state to
    /// nonsignaled. If this parameter is `false`, the function creates an auto-reset event
    /// object, and system automatically resets the event state to nonsignaled after a single
    /// waiting thread has been released.
    pub fn manual_reset(&mut self, manual_reset: bool) -> &mut EventBuilder {
        self.manual_reset = manual_reset;
        self
    }

    /// The initial state of the event.
    ///
    /// If the parameter is `true`, the initial state of the event object is signaled;
    /// otherwise, it is nonsignaled.
    pub fn initial_state(&mut self, initial_state: bool) -> &mut EventBuilder {
        self.initial_state = initial_state;
        self
    }

    /// The desired access for the event object.
    pub fn desired_access<A: Access>(&mut self, desired_access: A) -> &mut EventBuilder {
        self.desired_access = Some(desired_access.mask());
        self
    }
}

impl NamedBuilder for EventBuilder {
    type Output = Event;

    #[doc(hidden)]
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


/// A synchronization object whose state can be explicitly set to signaled.
///
/// See [Event Objects](https://msdn.microsoft.com/en-us/library/windows/desktop/ms682655%28v=vs.85%29.aspx).
#[derive(Debug)]
pub struct Event(Handle);

handle!(Event);

access! { EventAccess,
    ModifyState => c::EVENT_MODIFY_STATE;

    all => c::EVENT_ALL_ACCESS
}

impl Event {
    /// Creates a new anonymous event with default settings.
    pub fn create() -> io::Result<Event> {
        EventBuilder::new().create()
    }

    /// Creates a new named event with default settings.
    pub fn create_named<N: AsRef<OsStr>>(name: N) -> CreateNamedResult<Event> {
        EventBuilder::new().create_named(name)
    }

    /// Sets the event object to the signaled state.
    ///
    /// See [SetEvent](https://msdn.microsoft.com/en-us/library/windows/desktop/ms686211%28v=vs.85%29.aspx).
    pub fn set(&self) -> io::Result<()> {
        unsafe { check_bool(k32::SetEvent(self.as_raw_handle())) }
    }

    /// Sets the event object to the nonsignaled state.
    ///
    /// See [ResetEvent](https://msdn.microsoft.com/en-us/library/windows/desktop/ms685081%28v=vs.85%29.aspx).
    pub fn reset(&self) -> io::Result<()> {
        unsafe { check_bool(k32::ResetEvent(self.as_raw_handle())) }
    }
}

impl NamedObject for Event {
    fn open_function() -> NamedOpenFunction {
        k32::OpenEventW
    }

    fn default_open_options() -> NamedOpenOptions {
        EventAccess::all().into()
    }
}

impl Waitable for Event {}
