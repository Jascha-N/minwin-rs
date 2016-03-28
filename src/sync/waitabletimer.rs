use kernel32 as k32;
use chrono::{DateTime, Duration, NaiveDate, TimeZone, UTC};
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



pub struct WaitableTimerBuilder {
    security_attributes: Option<w::SECURITY_ATTRIBUTES>,
    manual_reset: bool,
    desired_access: Option<u32>,
}

impl WaitableTimerBuilder {
    #[cfg_attr(feature = "clippy", allow(new_without_default))]
    pub fn new() -> WaitableTimerBuilder {
        WaitableTimerBuilder {
            security_attributes: None,
            manual_reset: false,
            desired_access: None,
        }
    }

    pub fn manual_reset(&mut self, manual: bool) -> &mut WaitableTimerBuilder {
        self.manual_reset = manual;
        self
    }

    pub fn desired_access<A: Access>(&mut self, desired_access: A) -> &mut WaitableTimerBuilder {
        self.desired_access = Some(desired_access.mask());
        self
    }
}

impl NamedBuilder for WaitableTimerBuilder {
    type Output = WaitableTimer;

    fn __create_inner(&self, name: Option<WideString>) -> io::Result<(WaitableTimer, bool)> {
        let mut sa = self.security_attributes;
        let sa_ptr = sa.as_mut().map_or(ptr::null_mut(), |sa| sa);
        let name_ptr = name.as_ref().map_or(ptr::null(), |name| name.as_ptr());

        unsafe {
            let handle = match self.desired_access {
                Some(access) => {
                    let flags = if self.manual_reset {
                        c::CREATE_WAITABLE_TIMER_MANUAL_RESET
                    } else {
                        0
                    };
                    try!(check_handle(k32::CreateWaitableTimerExW(sa_ptr, name_ptr, flags, access)))
                }
                None => {
                    let manual_reset = if self.manual_reset {
                        w::TRUE
                    } else {
                        w::FALSE
                    };
                    try!(check_handle(k32::CreateWaitableTimerW(sa_ptr, manual_reset, name_ptr)))
                }
            };
            let created = k32::GetLastError() != w::ERROR_ALREADY_EXISTS;

            Ok((WaitableTimer::from_raw_handle(handle), created))
        }
    }
}


pub enum DueTime {
    Relative(Duration),
    Absolute(DateTime<UTC>),
}

impl DueTime {
    pub fn as_filetime(&self) -> i64 {
        match *self {
            DueTime::Relative(ref duration) => {
                let duration = duration.num_nanoseconds().expect("overflow") / 100;
                if duration < 0 {
                    panic!("negative duration")
                }
                -(duration as i64)
            }
            DueTime::Absolute(ref date_time) => {
                let naive = date_time.naive_utc();
                let base = NaiveDate::from_ymd(1601, 1, 1).and_hms(0, 0, 0);
                let diff = naive - base;
                diff.num_nanoseconds().expect("overflow") / 100
            }
        }
    }
}

object!(WaitableTimer);

access! { WaitableTimerAccess,
    ModifyState => c::TIMER_MODIFY_STATE,
    QueryState => c::TIMER_QUERY_STATE;

    all => c::TIMER_ALL_ACCESS
}

impl WaitableTimer {
    pub fn create() -> io::Result<WaitableTimer> {
        WaitableTimerBuilder::new().create()
    }

    pub fn create_named<N: AsRef<OsStr>>(name: N) -> CreateNamedResult<WaitableTimer> {
        WaitableTimerBuilder::new().create_named(name)
    }

    pub fn set(&self, due_time: DueTime, period: Option<Duration>) -> io::Result<()> {
        let due_time = due_time.as_filetime();
        let period = period.as_ref().map_or(0, Duration::num_milliseconds);

        unsafe {
            check_bool(k32::SetWaitableTimer(self.as_raw_handle(),
                                             &due_time,
                                             period as w::LONG,
                                             None,
                                             ptr::null_mut(),
                                             w::FALSE))
        }
    }

    // This is not the best :(
    pub unsafe fn set_with_completion_routine<F>(&self,
                                                 due_time: DueTime,
                                                 period: Option<Duration>,
                                                 completion_routine: &mut F)
                                                 -> io::Result<()>
        where F: FnMut(DateTime<UTC>)
    {
        unsafe extern "system" fn completion_routine_callback<F>(arg: w::LPVOID,
                                                                 timer_low: w::DWORD,
                                                                 timer_high: w::DWORD)
            where F: FnMut(DateTime<UTC>)
        {
            let completion_routine = &mut *(arg as *mut F);
            completion_routine(filetime_as_time((timer_high as i64) << 32 | timer_low as i64, UTC));
        }

        let due_time = due_time.as_filetime();
        let period = period.as_ref().map_or(0, Duration::num_milliseconds);

        check_bool(k32::SetWaitableTimer(self.as_raw_handle(),
                                         &due_time,
                                         period as w::LONG,
                                         Some(completion_routine_callback::<F>),
                                         completion_routine as *mut _ as w::LPVOID,
                                         w::FALSE))
    }

    pub fn cancel(&self) -> io::Result<()> {
        unsafe {
            check_bool(k32::CancelWaitableTimer(self.as_raw_handle()))
        }
    }
}

fn filetime_as_time<Tz: TimeZone>(file_time: i64, offset: Tz::Offset) -> DateTime<Tz> {
    let base = NaiveDate::from_ymd(1601, 1, 1).and_hms(0, 0, 0);
    let diff = Duration::nanoseconds(file_time.checked_mul(100).expect("overflow"));
    DateTime::from_utc(base + diff, offset)
}

impl NamedObject for WaitableTimer {
    fn __open_function() -> NamedOpenFunction {
        k32::OpenWaitableTimerW
    }
}

impl Waitable for WaitableTimer {}
