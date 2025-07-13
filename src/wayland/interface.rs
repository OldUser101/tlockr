use crate::wayland::{buffer::Buffer, keyboard::KeyboardMapping, state::LockState};

use wayland_client::{
    Connection, EventQueue,
    protocol::{
        wl_compositor::WlCompositor, wl_display::WlDisplay, wl_keyboard::WlKeyboard,
        wl_output::WlOutput, wl_registry::WlRegistry, wl_seat::WlSeat, wl_shm::WlShm,
        wl_surface::WlSurface,
    },
};
use wayland_protocols::{
    ext::session_lock::v1::client::{
        ext_session_lock_manager_v1::ExtSessionLockManagerV1,
        ext_session_lock_surface_v1::ExtSessionLockSurfaceV1,
        ext_session_lock_v1::ExtSessionLockV1,
    },
    wp::viewporter::client::{wp_viewport::WpViewport, wp_viewporter::WpViewporter},
};

pub struct WaylandInterfaces {
    pub connection: Option<Connection>,
    pub display: Option<WlDisplay>,
    pub registry: Option<WlRegistry>,

    pub output: Option<WlOutput>,
    pub compositor: Option<WlCompositor>,
    pub seat: Option<WlSeat>,
    pub viewporter: Option<WpViewporter>,

    pub surface: Option<WlSurface>,

    pub session_lock_manager: Option<ExtSessionLockManagerV1>,
    pub session_lock: Option<ExtSessionLockV1>,
    pub session_lock_surface: Option<ExtSessionLockSurfaceV1>,

    pub keyboard: Option<WlKeyboard>,

    pub keymap: Option<KeyboardMapping>,
    pub buffers: Option<Vec<Buffer>>,
    pub shm: Option<WlShm>,

    pub viewport: Option<WpViewport>,

    pub width: i32,
    pub height: i32,

    pub output_configured: bool,
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
            surface: None,
            session_lock_manager: None,
            session_lock: None,
            session_lock_surface: None,
            keyboard: None,
            keymap: None,
            buffers: Some(Vec::new()),
            shm: None,
            viewport: None,
            width: -1,
            height: -1,
            output_configured: false,
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

    pub fn ready(&self) -> bool {
        return self.output_configured;
    }
}
