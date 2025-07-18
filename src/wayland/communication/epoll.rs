use std::os::fd::RawFd;

use nix::sys::epoll::EpollFlags;

use crate::wayland::communication::event::EventType;

pub trait EpollMonitorable {
    fn get_fds(&self) -> Vec<(RawFd, EpollFlags, u64)>;
    fn handle_event(&mut self, event_type: EventType) -> Result<(), Box<dyn std::error::Error>>;
}
