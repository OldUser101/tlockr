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
    events_received: bool,
}

impl WaylandStateHandler {
    pub fn new(state: &mut WaylandState, event_queue: &mut EventQueue<WaylandState>) -> Self {
        Self {
            state: state as *mut WaylandState,
            event_queue: event_queue as *mut EventQueue<WaylandState>,
            read_guard: None,
            fd: None,
            events_received: false,
        }
    }
}

impl WaylandStateHandler {
    pub fn process_events(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.read_guard
            .take()
            .ok_or::<Box<dyn std::error::Error>>("Invalid read guard".into())?
            .read()?;

        let event_queue = unsafe { &mut *self.event_queue };
        let state = unsafe { &mut *self.state };
        event_queue.dispatch_pending(state)?;

        Ok(())
    }
}

impl EventHandler for WaylandStateHandler {
    fn event_type(&self) -> EventType {
        EventType::Wayland
    }

    fn prepare_handler(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.events_received = false;

        let event_queue = unsafe { &mut *self.event_queue };
        let state = unsafe { &mut *self.state };

        state.update_states(event_queue)?;
        event_queue.flush()?;
        event_queue.dispatch_pending(state)?;

        Ok(())
    }

    fn get_file_descriptor(
        &mut self,
        _comm_manager: Option<&CommunicationManager>,
    ) -> Option<BorrowedFd<'_>> {
        let event_queue = unsafe { &mut *self.event_queue };

        if let Some(read_guard) = event_queue.prepare_read() {
            let fd = read_guard.connection_fd();
            let raw_fd = fd.as_raw_fd();
            self.fd = Some(raw_fd);
            self.read_guard = Some(read_guard);
            return Some(unsafe { BorrowedFd::borrow_raw(raw_fd) });
        }

        None
    }

    fn notify_event(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.events_received = true;

        Ok(())
    }

    fn cleanup_handler(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.process_events()?;

        if let Some(read_guard) = self.read_guard.take() {
            drop(read_guard);
        }

        Ok(())
    }
}
