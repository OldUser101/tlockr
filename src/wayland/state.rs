use crate::wayland::renderer::QmlRendererInterface;
use crate::wayland::{interface::WaylandState, lock::State};
use nix::sys::eventfd::EventFd;
use wayland_client::EventQueue;

pub struct LockState {
    pub interfaces: WaylandState,
    pub renderer: QmlRendererInterface,
    pub state: State,
    pub qml_path: String,
    pub renderer_fd: Option<EventFd>,
}

impl LockState {
    pub fn new(qml_path: String) -> Self {
        Self {
            interfaces: WaylandState::new(),
            renderer: QmlRendererInterface::new(),
            state: State::None,
            qml_path: qml_path,
            renderer_fd: None,
        }
    }

    pub fn initialize(&mut self) -> Result<EventQueue<Self>, Box<dyn std::error::Error>> {
        let event_queue = self.interfaces.create_and_bind()?;
        self.state = State::Initialized;

        let renderer_fd = EventFd::new()?;
        self.renderer_fd = Some(renderer_fd);

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
        match self.state {
            State::Initialized => {
                if self.interfaces.ready() {
                    self.state = State::Ready;
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
