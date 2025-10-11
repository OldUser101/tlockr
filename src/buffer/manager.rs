// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025, Nathan Gill

/*
    manager.rs:
        Provides the `BufferManager` structure for alloocating and using pixel buffers.
*/

use crate::event::Event;
use crate::wayland::WaylandState;

use std::os::raw::c_void;
use tracing::error;
use wayland_client::protocol::wl_buffer;
use wayland_client::protocol::{wl_buffer::WlBuffer, wl_shm::WlShm};
use wayland_client::{Connection, Dispatch, QueueHandle};

/// Represents a buffer for storing pixel data
///
/// This structure contains a `WlBuffer` object for use in Wayland contexts,
/// as well as a raw pointer to the buffer's memory for use in external rendering
/// code.
///
/// The `in_use` member is a boolean value that dictates whether the buffer is currently
/// being used by the Wayland compositor. If this is set to `true`, the buffer's data
/// should not be modified, as this violates Wayland protocol.
pub struct Buffer {
    pub buffer: WlBuffer,
    pub in_use: bool,
    pub data: *mut u8,
}

/// `BufferManager` is a structure that can be used for allocating `Buffer` objects.
///
/// This structure contains various functions for operating on these buffers.
///
/// The `width`, `height`, and `shm` members need to be set before this structure is
/// ready to allocate buffers. This can be done via the `set_output_dimensions` and
/// `set_shm` methods respectively.
pub struct BufferManager {
    pub buffers: Option<Vec<Buffer>>,
    pub shm: Option<WlShm>,
    pub width: Option<i32>,
    pub height: Option<i32>,
}

impl BufferManager {
    /// Create a new `BufferManager`, with the buffers array initialized
    ///
    /// `set_output_dimensions` and `set_shm` need to be used before the `BufferManager` object is
    /// ready to allocate buffers.
    pub fn new() -> Self {
        Self {
            buffers: Some(Vec::new()),
            shm: None,
            width: None,
            height: None,
        }
    }

    pub fn set_output_dimensions(&mut self, width: i32, height: i32) {
        self.width = Some(width);
        self.height = Some(height);
    }

    pub fn set_shm(&mut self, shm: WlShm) {
        self.shm = Some(shm);
    }

    /// This function returns the dimensions of a display buffer in the format `(width, height, stride, size)`.
    pub fn calculate_buffer_dimensions(
        &self,
    ) -> Result<(i32, i32, i32, i32), Box<dyn std::error::Error>> {
        let width = self
            .width
            .ok_or::<Box<dyn std::error::Error>>("Invalid width".into())?;
        let height = self
            .height
            .ok_or::<Box<dyn std::error::Error>>("Invalid height".into())?;
        let stride = width * 4;

        Ok((width, height, stride, height * stride))
    }

    /// Release a buffer by index
    ///
    /// This function sets the `in_use` member of the buffer specified by `index` to `false`.
    /// This function is intended to be used after receiving the `wl_buffer::Event::Release` event,
    /// signalling that the specified buffer has been released.
    pub fn release_buffer(&mut self, index: usize) -> Result<(), Box<dyn std::error::Error>> {
        let buffers = self
            .buffers
            .as_mut()
            .ok_or::<Box<dyn std::error::Error>>("Failed to obtain buffers array".into())?;

        if index >= buffers.len() {
            return Err("Buffer index out of range".into());
        }

        buffers[index].in_use = false;

        Ok(())
    }

    /// Returns the next available buffer from the buffer store
    pub fn find_available_buffer(&self) -> Option<&Buffer> {
        self.buffers.as_ref()?.iter().find(|b| !b.in_use)
    }

    /// Searches for a `Buffer` matching the data provided by a `RendererEvent`
    pub fn find_buffer_from_event(
        &mut self,
        event: &Event,
    ) -> Result<&mut Buffer, Box<dyn std::error::Error>> {
        let buffers = self.buffers.as_mut().ok_or("Buffers unavailable")?;

        buffers
            .iter_mut()
            .find(|b| b.data as *mut c_void == event.param_1.into())
            .ok_or("No matching buffer found".into())
    }
}

impl Dispatch<WlBuffer, i32> for WaylandState {
    fn event(
        state: &mut Self,
        _proxy: &WlBuffer,
        event: <WlBuffer as wayland_client::Proxy>::Event,
        data: &i32,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
    ) {
        match event {
            wl_buffer::Event::Release => {
                // Release the buffer so it can be used again
                if let Err(e) = state.buffer_manager.release_buffer(*data as usize) {
                    error!("{}", e);
                }
            }
            _ => {}
        }
    }
}
