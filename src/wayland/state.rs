use crate::shared::interface::{get_state, set_renderer_fd};
use crate::shared::{interface::set_state, state::State};
use crate::wayland::interface::WaylandState;
use nix::sys::eventfd::EventFd;
use std::os::fd::AsRawFd;
use wayland_client::EventQueue;

impl WaylandState {
    pub fn initialize(&mut self) -> Result<EventQueue<Self>, Box<dyn std::error::Error>> {
        let event_queue = self.create_and_bind()?;

        set_state(self.app_state, State::Initialized);

        unsafe {
            (*self.app_state).state = State::Initialized;
        };

        let renderer_fd = EventFd::new()?;
        set_renderer_fd(self.app_state, renderer_fd.as_raw_fd());
        self.renderer_event_fd = Some(renderer_fd);

        Ok(event_queue)
    }

    pub fn roundtrip(
        &mut self,
        event_queue: &mut EventQueue<Self>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        event_queue.roundtrip(self)?;
        Ok(())
    }

    pub fn update_states(
        &mut self,
        event_queue: &EventQueue<Self>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match get_state(self.app_state).unwrap() {
            State::Initialized => {
                if self.ready() {
                    set_state(self.app_state, State::Ready);
                }
            }
            State::Ready => {
                self.allocate_buffers(event_queue, 2)?;
                self.initialize_renderer()?;
                self.lock(event_queue)?;
            }
            _ => {}
        }

        Ok(())
    }
}
