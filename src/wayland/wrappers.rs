use std::os::fd::{AsRawFd, BorrowedFd, RawFd};

use wayland_client::{EventQueue, backend::ReadEventsGuard};

use crate::wayland::{
    communication::manager::CommunicationManager,
    event::{event::EventType, handler::EventHandler},
    state::WaylandState,
};

pub struct WaylandStateHandler {
    state: *mut WaylandState,
    event_queue: *mut EventQueue<WaylandState>,
    read_guard: Option<ReadEventsGuard>,
    fd: Option<RawFd>,
}

impl WaylandStateHandler {
    pub fn new(state: &mut WaylandState, event_queue: &mut EventQueue<WaylandState>) -> Self {
        Self {
            state: state as *mut WaylandState,
            event_queue: event_queue as *mut EventQueue<WaylandState>,
            read_guard: None,
            fd: None,
        }
    }
}

impl EventHandler for WaylandStateHandler {
    fn event_type(&self) -> EventType {
        EventType::Wayland
    }

    fn prepare_handler(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let event_queue = unsafe { &mut *self.event_queue };
        self.read_guard = event_queue.prepare_read();

        Ok(())
    }

    fn get_file_descriptor(
        &mut self,
        _comm_manager: Option<&CommunicationManager>,
    ) -> Option<BorrowedFd<'_>> {
        if let Some(read_guard) = &self.read_guard {
            let fd = read_guard.connection_fd();
            self.fd = Some(fd.as_raw_fd());
            return Some(fd);
        }

        None
    }

    fn notify_event(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.read_guard
            .take()
            .ok_or::<Box<dyn std::error::Error>>("Invalid read guard".into())?
            .read()?;

        let event_queue = unsafe { &mut *self.event_queue };
        let state = unsafe { &mut *self.state };
        event_queue.dispatch_pending(state)?;

        Ok(())
    }

    fn cleanup_handler(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(read_guard) = self.read_guard.take() {
            drop(read_guard);
        }

        Ok(())
    }
}
