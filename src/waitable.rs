use chrono::Duration;
use kernel32 as k32;
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::io;
use winapi as w;

use object::Object;



#[derive(Debug)]
pub enum WaitError {
    Abandoned(usize),
    Timeout,
    Io(io::Error),
}

impl Error for WaitError {
    fn description(&self) -> &str {
        match *self {
            WaitError::Abandoned(_) => "abandoned mutex",
            WaitError::Timeout => "wait timeout",
            WaitError::Io(_) => "I/O error"
        }
    }
}

impl Display for WaitError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match *self {
            WaitError::Abandoned(_) => write!(formatter, "A thread that owned a mutex terminated"),
            WaitError::Timeout => write!(formatter, "A wait timed out"),
            WaitError::Io(ref error) => write!(formatter, "An I/O error occurred: {}", error)
        }
    }
}

impl From<io::Error> for WaitError {
    fn from(error: io::Error) -> WaitError {
        WaitError::Io(error)
    }
}



pub type WaitResult<T> = Result<T, WaitError>;

pub trait Waitable: Object {
    fn wait(&self) -> WaitResult<()> {
        wait_for_single(self, None)
    }

    fn wait_timeout(&self, timeout: Duration) -> WaitResult<()> {
        wait_for_single(self, Some(timeout))
    }
}

fn wait_for_single<W: ?Sized + Waitable>(waitable: &W,
                                         timeout: Option<Duration>)
                                         -> WaitResult<()> {
    unsafe {
        match k32::WaitForSingleObject(waitable.as_raw_handle(),
                                       timeout.map_or(w::INFINITE, |timeout| {
                                           timeout.num_milliseconds() as w::DWORD
                                       })) {
            w::WAIT_ABANDONED => Err(WaitError::Abandoned(0)),
            w::WAIT_TIMEOUT => Err(WaitError::Timeout),
            w::WAIT_FAILED => Err(WaitError::Io(io::Error::last_os_error())),
            _ => Ok(()),
        }
    }
}

#[cfg_attr(feature = "clippy", allow(absurd_extreme_comparisons))]
fn wait_for_multiple(objects: &[&Waitable],
                     timeout: Option<Duration>,
                     wait_all: bool)
                     -> WaitResult<usize> {
    let handles = objects.iter().map(|object| object.as_raw_handle()).collect::<Vec<_>>();
    let count = handles.len() as w::DWORD;
    let wait_all = if wait_all {
        w::TRUE
    } else {
        w::FALSE
    };

    unsafe {
        match k32::WaitForMultipleObjects(count,
                                          handles.as_ptr(),
                                          wait_all,
                                          timeout.map_or(w::INFINITE, |timeout| {
                                              timeout.num_milliseconds() as w::DWORD
                                          })) {
            w::WAIT_TIMEOUT => Err(WaitError::Timeout),
            w::WAIT_FAILED => Err(WaitError::Io(io::Error::last_os_error())),
            value if value >= w::WAIT_OBJECT_0 && value < w::WAIT_OBJECT_0 + count => {
                Ok((value - w::WAIT_OBJECT_0) as usize)
            }
            value => Err(WaitError::Abandoned((value - w::WAIT_ABANDONED_0) as usize)),
        }
    }
}

pub fn wait_for_all(objects: &[&Waitable], timeout: Option<Duration>) -> WaitResult<usize> {
    wait_for_multiple(objects, timeout, true)
}

pub fn wait_for_any(objects: &[&Waitable], timeout: Option<Duration>) -> WaitResult<usize> {
    wait_for_multiple(objects, timeout, false)
}
