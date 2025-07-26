use std::os::fd::BorrowedFd;

use crate::wayland::{
    communication::{channel::CommunicationChannel, manager::CommunicationManager},
    event::{event::EventType, handler::EventHandler},
    state::WaylandState,
};

pub struct RendererHandler {
    state: *mut WaylandState,
    channel: Option<*const CommunicationChannel>,
}

impl RendererHandler {
    pub fn new(state: &mut WaylandState) -> Self {
        Self {
            state: state as *mut WaylandState,
            channel: None,
        }
    }
}

impl EventHandler for RendererHandler {
    fn event_type(&self) -> EventType {
        EventType::Renderer
    }

    fn get_file_descriptor(
        &mut self,
        comm_manager: Option<&CommunicationManager>,
    ) -> Option<BorrowedFd<'_>> {
        let channel = comm_manager
            .unwrap()
            .get_channel(EventType::Renderer)
            .unwrap();
        self.channel = Some(channel as *const CommunicationChannel);

        let state = unsafe { &mut *self.state };
        let fd = state.renderer_read_fd.unwrap();
        Some(unsafe { BorrowedFd::borrow_raw(fd) })
    }

    fn notify_event(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let state = unsafe { &mut *self.state };
        let channel = unsafe {
            &*self
                .channel
                .ok_or::<Box<dyn std::error::Error>>("No renderer channel".into())?
        };
        let event = channel.receive_event()?;
        state.update_buffer(&event)?;
        Ok(())
    }
}
