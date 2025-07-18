use std::os::fd::{AsRawFd, RawFd};

use crate::wayland::communication::event::{Event, EventType};
use crate::wayland::communication::pipe::Pipe;

pub struct CommunicationChannel {
    _event_type: EventType,
    pipe: Pipe,
}

impl CommunicationChannel {
    pub fn new(_event_type: EventType) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            _event_type,
            pipe: Pipe::new()?,
        })
    }

    pub fn send_event(&self, event: Event) -> Result<(), Box<dyn std::error::Error>> {
        let event_bytes = unsafe {
            std::slice::from_raw_parts(
                &event as *const Event as *const u8,
                std::mem::size_of::<Event>(),
            )
        };

        let write_fd = self.pipe.write_fd();

        let bytes_written = nix::unistd::write(write_fd, &event_bytes)?;
        if bytes_written != std::mem::size_of::<Event>() {
            return Err("Failed to write event bytes".into());
        }

        Ok(())
    }

    pub fn receive_event(&self) -> Result<Event, Box<dyn std::error::Error>> {
        let mut event_bytes = [0u8; std::mem::size_of::<Event>()];
        let read_fd = self.pipe.read_fd();

        let bytes_read = nix::unistd::read(read_fd, &mut event_bytes)?;
        if bytes_read != std::mem::size_of::<Event>() {
            return Err("Failed to read event bytes".into());
        }

        let event = unsafe { std::ptr::read(event_bytes.as_ptr() as *const Event) };

        Ok(event)
    }

    pub fn read_fd_raw(&self) -> RawFd {
        self.pipe.read_fd().as_raw_fd()
    }

    pub fn write_fd_raw(&self) -> RawFd {
        self.pipe.write_fd().as_raw_fd()
    }
}
