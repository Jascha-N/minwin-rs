// Original by Alex Crichton
use std::{mem, ptr};
use std::os::windows::io::AsRawHandle;
use winapi as w;

use sync::Event;



#[derive(Debug)]
pub struct Overlapped {
    inner: w::OVERLAPPED,
    event: Option<Event>,
}

impl Overlapped {
    pub fn new() -> Overlapped {
        Overlapped {
            inner: unsafe { mem::zeroed() },
            event: None,
        }
    }

    pub fn get(&mut self) -> &mut w::OVERLAPPED {
        &mut self.inner
    }

    pub fn set_offset(&mut self, offset: u64) {
        self.inner.Offset = offset as u32;
        self.inner.OffsetHigh = (offset >> 32) as u32;
    }

    pub fn offset(&self) -> u64 {
        (self.inner.Offset as u64) | ((self.inner.OffsetHigh as u64) << 32)
    }

    pub fn set_event(&mut self, event: Option<Event>) {
        self.inner.hEvent = event.as_ref().map_or(ptr::null_mut(), Event::as_raw_handle);
        self.event = event;
    }

    pub fn event(&self) -> Option<&Event> {
        self.event.as_ref()
    }
}

impl Default for Overlapped {
    fn default() -> Overlapped {
        Overlapped::new()
    }
}

unsafe impl Send for Overlapped {}
unsafe impl Sync for Overlapped {}
