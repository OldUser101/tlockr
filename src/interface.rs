use crate::state::LockState;

use wayland_client::{
    Connection, EventQueue,
    protocol::{
        wl_compositor::WlCompositor, wl_display::WlDisplay, wl_output::WlOutput,
        wl_registry::WlRegistry, wl_seat::WlSeat,
    },
};
use wayland_protocols::{
    ext::session_lock::v1::client::ext_session_lock_manager_v1::ExtSessionLockManagerV1,
    wp::viewporter::client::wp_viewporter::WpViewporter,
};

pub struct WaylandInterfaces {
    pub connection: Option<Connection>,
    pub display: Option<WlDisplay>,
    pub registry: Option<WlRegistry>,

    pub output: Option<WlOutput>,
    pub compositor: Option<WlCompositor>,
    pub seat: Option<WlSeat>,
    pub viewporter: Option<WpViewporter>,

    pub session_lock_manager: Option<ExtSessionLockManagerV1>,
}

impl WaylandInterfaces {
    pub fn new() -> Self {
        Self {
            connection: None,
            display: None,
            registry: None,
            output: None,
            compositor: None,
            seat: None,
            viewporter: None,
            session_lock_manager: None,
        }
    }

    pub fn create_and_bind(&mut self) -> Result<EventQueue<LockState>, Box<dyn std::error::Error>> {
        let conn = Connection::connect_to_env()?;
        let display = conn.display();

        let event_queue = conn.new_event_queue();
        let qh = event_queue.handle();

        let registry = display.get_registry(&qh, ());

        self.connection = Some(conn);
        self.display = Some(display);
        self.registry = Some(registry);

        Ok(event_queue)
    }
}
