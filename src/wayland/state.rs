// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025, Nathan Gill

use crate::shared::interface::{get_state, set_renderer_fd};
use crate::shared::{interface::set_state, state::State};
use crate::{
    shared::state::ApplicationState,
    wayland::{graphics::buffer::Buffer, input::keyboard::KeyboardMapping},
};
use nix::sys::eventfd::EventFd;
use std::os::fd::AsRawFd;
use wayland_client::EventQueue;
use wayland_client::{
    Connection,
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

pub struct WaylandState {
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

    pub app_state: *mut ApplicationState,

    pub renderer_event_fd: Option<EventFd>,
    pub wayland_event_fd: Option<EventFd>,
}

impl WaylandState {
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
            app_state: app_state,
            renderer_event_fd: None,
            wayland_event_fd: None,
        }
    }

    pub fn create_and_bind(
        &mut self,
    ) -> Result<EventQueue<WaylandState>, Box<dyn std::error::Error>> {
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
