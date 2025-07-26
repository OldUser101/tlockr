// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025, Nathan Gill

/*
    renderer.rs:
        Initializes the QML renderer and provides callbacks for the renderer to
        trigger content refresh.
*/

use wayland_client::protocol::wl_surface::WlSurface;
use wayland_protocols::wp::viewporter::client::wp_viewport::WpViewport;

use crate::{
    shared::{
        ffi::{cleanup_renderer, initialize_renderer, set_callbacks},
        interface::{get_qml_path, get_renderer, set_renderer},
    },
    wayland::{
        buffer::manager::BufferManager,
        event::{
            event::{Event, EventType},
            handler::EventHandler,
        },
        state::WaylandState,
    },
};
use std::{ffi::c_void, i32};

impl WaylandState {
    /// Callback function for the renderer.
    ///
    /// This functions returns a pointer to the next available buffer.
    /// This tells the renderer where to render its content.
    unsafe extern "C" fn get_buffer_callback(user_data: *mut c_void) -> *mut c_void {
        let buffer_manager = user_data as *mut BufferManager;
        unsafe {
            buffer_manager
                .as_ref()
                .and_then(|bm| bm.find_available_buffer())
                .map(|b| b.data as *mut c_void)
                .unwrap_or(std::ptr::null_mut())
        }
    }

    /// Retrieves the currently active surface and viewport
    fn get_surface_and_viewport(
        &self,
    ) -> Result<(&WlSurface, &WpViewport), Box<dyn std::error::Error>> {
        match (&self.surface, &self.viewport) {
            (Some(surface), Some(viewport)) => Ok((surface, viewport)),
            _ => Err("Surface or viewport unavailable".into()),
        }
    }

    /// Updates a buffer associated with a renderer event
    ///
    /// This functions takes a `RendererEvent` object, and finds a `Buffer` associated with it.
    /// This buffer is then attached to the currently active Wayland surface, and committed.
    fn update_buffer(&mut self, event: &Event) -> Result<(), Box<dyn std::error::Error>> {
        let width = self.width;
        let height = self.height;

        let (surface_ptr, viewport_ptr) = {
            let (surface, viewport) = self.get_surface_and_viewport()?;
            (surface as *const WlSurface, viewport as *const WpViewport)
        };
        let buffer = self.buffer_manager.find_buffer_from_event(event)?;

        let surface = unsafe { &*surface_ptr };
        let viewport = unsafe { &*viewport_ptr };

        surface.attach(Some(&buffer.buffer), 0, 0);
        surface.damage_buffer(0, 0, i32::MAX, i32::MAX);
        viewport.set_destination(width, height);
        buffer.in_use = true;
        surface.commit();

        Ok(())
    }

    /// Initialize the renderer
    ///
    /// This function creates a new `QmlRenderer` object with the appropriate callbacks set for rendering.
    /// The QML content path is obtained from the current application state.
    /// The created renderer is stored in the current application state.
    pub fn initialize_renderer(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let qml_path = get_qml_path(self.app_state).ok_or("Failed to get QML path")?;

        let renderer =
            unsafe { initialize_renderer(self.width, self.height, qml_path, self.app_state) };

        if renderer.is_null() {
            return Err("QML renderer initialization failed.".into());
        }

        set_renderer(self.app_state, renderer);

        unsafe {
            set_callbacks(
                renderer,
                Self::get_buffer_callback,
                &mut self.buffer_manager as *mut _ as *mut c_void,
            );
        }

        Ok(())
    }

    /// Clean up the renderer object and thread
    pub fn destroy_renderer(&mut self) {
        if let Some(renderer) = get_renderer(self.app_state) {
            unsafe {
                cleanup_renderer(renderer);
                set_renderer(self.app_state, std::ptr::null_mut());
            }
        }
    }
}

impl EventHandler for &mut WaylandState {
    fn event_type(&self) -> EventType {
        EventType::Renderer
    }

    fn handle_event(&mut self, event: &Event) -> Result<(), Box<dyn std::error::Error>> {
        self.update_buffer(event)?;
        Ok(())
    }
}
