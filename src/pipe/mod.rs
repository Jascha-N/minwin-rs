use kernel32 as k32;
use std::{io, mem, ptr};
use std::ffi::{OsStr, OsString};
use std::os::windows::io::AsRawHandle;
use winapi as w;

use object::{Readable, Object, Writable};
use overlapped::Overlapped;
use string::FromWideString;
use util::*;

pub use self::anonymous::*;

mod anonymous;

pub trait Pipe: Object {
    // fn read_message(&self, buffer: &mut Vec<u8>) -> io::Result<usize> {

    // }

    fn kind(&self) -> io::Result<PipeType> {
        self.info().map(|info| info.kind())
    }

    fn transact(&self, in_buffer: &[u8], out_buffer: &mut [u8]) -> io::Result<u32>
        where Self: Readable + Writable
    {
        let in_size = buffer_size_dword(in_buffer);
        let out_size = buffer_size_dword(out_buffer);
        unsafe {
            let mut read = mem::uninitialized();

            try!(check_bool(k32::TransactNamedPipe(self.as_raw_handle(),
                                                   in_buffer.as_ptr() as w::LPVOID,
                                                   in_size,
                                                   out_buffer.as_mut_ptr() as w::LPVOID,
                                                   out_size,
                                                   &mut read,
                                                   ptr::null_mut())));

            Ok(read)
        }
    }

    unsafe fn transact_overlapped(&self,
                                  in_buffer: &[u8],
                                  out_buffer: &mut [u8],
                                  overlapped: &mut Overlapped)
                                  -> io::Result<bool>
        where Self: Readable + Writable
    {
        let in_size = buffer_size_dword(in_buffer);
        let out_size = buffer_size_dword(out_buffer);
        let result = check_bool(k32::TransactNamedPipe(self.as_raw_handle(),
                                                       in_buffer.as_ptr() as w::LPVOID,
                                                       in_size,
                                                       out_buffer.as_mut_ptr() as w::LPVOID,
                                                       out_size,
                                                       ptr::null_mut(),
                                                       overlapped.get()));

        match result {
            Ok(_) => Ok(true),
            Err(ref error) if error.raw_os_error() == Some(w::ERROR_IO_PENDING as i32) => Ok(false),
            Err(error) => Err(error),
        }
    }

    fn peek(&self, mut buffer: Option<&mut [u8]>) -> io::Result<PeekInfo>
        where Self: Readable
    {
        let (ptr, size) = match buffer {
            Some(ref mut buffer) => (buffer.as_mut_ptr(), buffer_size_dword(buffer)),
            None => (ptr::null_mut(), 0),
        };

        unsafe {
            let mut result = mem::uninitialized::<PeekInfo>();
            try!(check_bool(k32::PeekNamedPipe(self.as_raw_handle(),
                                               ptr as w::LPVOID,
                                               size,
                                               &mut result.bytes_read,
                                               &mut result.bytes_left,
                                               &mut result.bytes_message_left)));
            Ok(result)
        }
    }

    fn client_computer_name(&self) -> io::Result<OsString> {
        unsafe {
            let mut computer_name: [w::WCHAR; w::CNLEN as usize + 1] = mem::uninitialized();
            try!(check_bool(k32::GetNamedPipeClientComputerNameW(self.as_raw_handle(),
                                                                 computer_name.as_mut_ptr(),
                                                                 computer_name.len() as w::ULONG)));
            Ok(OsString::from_wide_string_null(&computer_name[..]))
        }
    }

    fn client_process_id(&self) -> io::Result<u32> {
        unsafe {
            let mut process_id = mem::uninitialized();
            try!(check_bool(k32::GetNamedPipeClientProcessId(self.as_raw_handle(),
                                                             &mut process_id)));
            Ok(process_id)
        }
    }

    fn client_session_id(&self) -> io::Result<u32> {
        unsafe {
            let mut session_id = mem::uninitialized();
            try!(check_bool(k32::GetNamedPipeClientSessionId(self.as_raw_handle(),
                                                             &mut session_id)));
            Ok(session_id)
        }
    }

    fn server_process_id(&self) -> io::Result<u32> {
        unsafe {
            let mut process_id = mem::uninitialized();
            try!(check_bool(k32::GetNamedPipeServerProcessId(self.as_raw_handle(),
                                                             &mut process_id)));
            Ok(process_id)
        }
    }

    fn server_session_id(&self) -> io::Result<u32> {
        unsafe {
            let mut session_id = mem::uninitialized();
            try!(check_bool(k32::GetNamedPipeServerSessionId(self.as_raw_handle(),
                                                             &mut session_id)));
            Ok(session_id)
        }
    }

    fn info(&self) -> io::Result<PipeInfo> {
        unsafe {
            let mut flags = mem::uninitialized();
            let mut out_buffer_size = mem::uninitialized();
            let mut in_buffer_size = mem::uninitialized();
            let mut max_instances = mem::uninitialized();

            try!(check_bool(k32::GetNamedPipeInfo(self.as_raw_handle(),
                                                  &mut flags,
                                                  &mut out_buffer_size,
                                                  &mut in_buffer_size,
                                                  &mut max_instances)));

            Ok(PipeInfo {
                kind: if flags & w::PIPE_TYPE_MESSAGE != 0 {
                    PipeType::Message
                } else {
                    PipeType::Byte
                },
                end: if flags & w::PIPE_SERVER_END != 0 {
                    PipeEnd::Server
                } else {
                    PipeEnd::Client
                },
                out_buffer_size: out_buffer_size,
                in_buffer_size: in_buffer_size,
                max_instances: if max_instances == w::PIPE_UNLIMITED_INSTANCES {
                    None
                } else {
                    Some(max_instances)
                },
            })
        }
    }

    // TODO: Check server/client
    fn state(&self) -> io::Result<PipeState> {
        unsafe {
            let mut state = mem::uninitialized();
            let mut current_instances = mem::uninitialized();
            let mut max_collection_count = mem::uninitialized();
            let mut collect_data_timeout = mem::uninitialized();
            let mut user_name: [w::WCHAR; w::UNLEN as usize + 1] = mem::uninitialized();

            try!(check_bool(k32::GetNamedPipeHandleStateW(self.as_raw_handle(),
                                                          &mut state,
                                                          &mut current_instances,
                                                          &mut max_collection_count,
                                                          &mut collect_data_timeout,
                                                          user_name.as_mut_ptr(),
                                                          user_name.len() as w::DWORD)));

            Ok(PipeState {
                blocking: if state & w::PIPE_NOWAIT != 0 {
                    true
                } else {
                    false
                },
                read_mode: if state & w::PIPE_READMODE_MESSAGE != 0 {
                    ReadMode::Message
                } else {
                    ReadMode::Byte
                },
                current_instances: current_instances,
                max_collection_count: max_collection_count,
                collect_data_timeout: collect_data_timeout,
                user_name: OsString::from_wide_string_null(&user_name[..]),
            })
        }
    }

    // TODO: Check server/client
    fn set_state(&self,
                 blocking: Option<bool>,
                 read_mode: Option<ReadMode>,
                 mut max_collection_count: Option<u32>,
                 mut collect_data_timeout: Option<u32>)
                 -> io::Result<()> {
        let blocking_flag = blocking.map(|blocking| {
            if blocking {
                w::PIPE_WAIT
            } else {
                w::PIPE_NOWAIT
            }
        });
        let read_mode_flag = read_mode.map(|read_mode| {
            match read_mode {
                ReadMode::Byte => w::PIPE_READMODE_BYTE,
                ReadMode::Message => w::PIPE_READMODE_MESSAGE,
            }
        });
        let mut mode = match (blocking_flag, read_mode_flag) {
            (Some(blocking_flag), Some(read_mode_flag)) => Some(blocking_flag | read_mode_flag),
            (Some(blocking_flag), None) => Some(blocking_flag),
            (None, Some(read_mode_flag)) => Some(read_mode_flag),
            (None, None) => None,
        };

        unsafe {
            check_bool(k32::SetNamedPipeHandleState(self.as_raw_handle(),
                                                    mode.as_mut()
                                                        .map_or(ptr::null_mut(), |mode| mode),
                                                    max_collection_count.as_mut()
                                                                        .map_or(ptr::null_mut(),
                                                                                |mcc| mcc),
                                                    collect_data_timeout.as_mut()
                                                                        .map_or(ptr::null_mut(),
                                                                                |cdt| cdt)))
        }
    }
}



#[derive(Debug, Clone)]
pub struct PipeInfo {
    kind: PipeType,
    end: PipeEnd,
    out_buffer_size: u32,
    in_buffer_size: u32,
    max_instances: Option<u32>,
}

impl PipeInfo {
    pub fn kind(&self) -> PipeType {
        self.kind
    }

    pub fn end(&self) -> PipeEnd {
        self.end
    }

    pub fn out_buffer_size(&self) -> u32 {
        self.out_buffer_size
    }

    pub fn in_buffer_size(&self) -> u32 {
        self.in_buffer_size
    }

    pub fn max_instances(&self) -> Option<u32> {
        self.max_instances
    }
}



#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum PipeType {
    Byte,
    Message,
}



#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum PipeEnd {
    Client,
    Server,
}



#[derive(Debug, Clone)]
pub struct PipeState {
    blocking: bool,
    read_mode: ReadMode,
    current_instances: u32,
    max_collection_count: u32,
    collect_data_timeout: u32,
    user_name: OsString,
}

impl PipeState {
    pub fn blocking(&self) -> bool {
        self.blocking
    }

    pub fn read_mode(&self) -> ReadMode {
        self.read_mode
    }

    pub fn current_instances(&self) -> u32 {
        self.current_instances
    }

    pub fn max_collection_count(&self) -> u32 {
        self.max_collection_count
    }

    pub fn collect_data_timeout(&self) -> u32 {
        self.collect_data_timeout
    }

    pub fn user_name(&self) -> &OsStr {
        &self.user_name
    }
}



#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ReadMode {
    Byte,
    Message,
}



#[derive(Debug, Clone)]
pub struct PeekInfo {
    bytes_read: u32,
    bytes_left: u32,
    bytes_message_left: u32,
}

impl PeekInfo {
    pub fn bytes_read(&self) -> u32 {
        self.bytes_read
    }

    pub fn bytes_left(&self) -> u32 {
        self.bytes_left
    }

    pub fn bytes_message_left(&self) -> u32 {
        self.bytes_message_left
    }
}
