// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025, Nathan Gill

/*
    state.rs:
        Defines the WaylandState object, which holds the state of the Wayland
        backend, and links with the rest of the application.
*/

use crate::shared::interface::{get_state, set_renderer_read_fd, set_renderer_write_fd};
use crate::shared::state::ApplicationState;
use crate::shared::{interface::set_state, state::State};
use crate::wayland::buffer::manager::BufferManager;
use crate::wayland::communication::pipe::Pipe;
use crate::wayland::event::event::Event;
use std::os::fd::AsRawFd;
use std::time::Instant;
use wayland_client::EventQueue;
use wayland_client::protocol::wl_pointer::WlPointer;
use wayland_client::{
    Connection,
    protocol::{
        wl_compositor::WlCompositor, wl_display::WlDisplay, wl_keyboard::WlKeyboard,
        wl_output::WlOutput, wl_registry::WlRegistry, wl_seat::WlSeat, wl_surface::WlSurface,
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

/// Main state structure
///
/// Holds all state information for the application
pub struct WaylandState {
    pub connection: Option<Connection>,
    pub display: Option<WlDisplay>,
    pub registry: Option<WlRegistry>,

    pub output: Option<WlOutput>,
    pub compositor: Option<WlCompositor>,
    pub seat: Option<WlSeat>,
    pub viewporter: Option<WpViewporter>,

    pub surface: Option<WlSurface>,

    pub buffer_manager: BufferManager,

    pub session_lock_manager: Option<ExtSessionLockManagerV1>,
    pub session_lock: Option<ExtSessionLockV1>,
    pub session_lock_surface: Option<ExtSessionLockSurfaceV1>,

    pub keyboard: Option<WlKeyboard>,
    pub pointer: Option<WlPointer>,

    pub viewport: Option<WpViewport>,

    pub width: i32,
    pub height: i32,

    pub output_configured: bool,

    pub app_state: *mut ApplicationState,

    pub renderer_read_pipe: Option<Pipe>,
    pub renderer_write_pipe: Option<Pipe>,

    pub pointer_timestamp: Option<Instant>,
    pub pending_pointer_event: Option<Event>,
}

impl WaylandState {
    /// Create a new `WaylandState`
    ///
    /// `initialize` needs to be called before the returned object is usable
    pub fn new(app_state: *mut ApplicationState) -> Self {
        Self {
            connection: None,
            display: None,
            registry: None,
            output: None,
            compositor: None,
            seat: None,
            viewporter: None,
            surface: None,
            buffer_manager: BufferManager::new(),
            session_lock_manager: None,
            session_lock: None,
            session_lock_surface: None,
            keyboard: None,
            pointer: None,
            viewport: None,
            width: -1,
            height: -1,
            output_configured: false,
            app_state: app_state,
            renderer_read_pipe: None,
            renderer_write_pipe: None,
            pointer_timestamp: None,
            pending_pointer_event: None,
        }
    }

    /// Open a Wayland connection and bind to the display and registry
    fn create_and_bind(&mut self) -> Result<EventQueue<WaylandState>, Box<dyn std::error::Error>> {
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

    /// Returns a boolean value indicating whether this object instance is ready for use
    pub fn ready(&self) -> bool {
        return self.output_configured;
    }

    /// Prepare the object for use
    ///
    /// This function establishes an `EventQueue` and prepares renderer communication.
    pub fn initialize(&mut self) -> Result<EventQueue<Self>, Box<dyn std::error::Error>> {
        let event_queue = self.create_and_bind()?;

        set_state(self.app_state, State::Initialized);

        let renderer_read_pipe = Pipe::new()?;
        let renderer_write_pipe = Pipe::new()?;

        // The naming is weird here
        // The renderer's read fd is the readable side of our write pipe
        // The renderer's write fd is the writeable side of our read pipe
        set_renderer_read_fd(self.app_state, renderer_write_pipe.read_fd().as_raw_fd());
        set_renderer_write_fd(self.app_state, renderer_read_pipe.write_fd().as_raw_fd());

        self.renderer_read_pipe = Some(renderer_read_pipe);
        self.renderer_write_pipe = Some(renderer_write_pipe);

        Ok(event_queue)
    }

    /// Wrapper for an event queue roundtrip
    pub fn roundtrip(
        &mut self,
        event_queue: &mut EventQueue<Self>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        event_queue.roundtrip(self)?;
        Ok(())
    }

    /// Update and control the lock based on its current state
    pub fn update_states(
        &mut self,
        event_queue: &EventQueue<Self>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match get_state(self.app_state)
            .ok_or::<Box<dyn std::error::Error>>("Failed to get application state".into())?
        {
            State::Initialized => {
                if self.ready() {
                    set_state(self.app_state, State::Ready);
                }
            }
            State::Ready => {
                self.buffer_manager
                    .set_output_dimensions(self.width, self.height);
                self.buffer_manager.allocate_buffers(event_queue, 2)?;
                self.initialize_renderer()?;
                self.lock(event_queue)?;
            }
            _ => {}
        }

        Ok(())
    }
}
