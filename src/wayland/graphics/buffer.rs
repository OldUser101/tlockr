// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025, Nathan Gill

/*
    buffer.rs:
        Buffer allocation and management for the renderer.
*/

use crate::wayland::state::WaylandState;
use nix::libc::{MAP_SHARED, PROT_READ, PROT_WRITE, ftruncate, mmap};
use nix::sys::memfd::{MFdFlags, memfd_create};
use std::os::fd::{AsFd, AsRawFd, OwnedFd, RawFd};
use std::os::raw::c_void;
use wayland_client::EventQueue;
use wayland_client::protocol::wl_shm;
use wayland_client::protocol::wl_shm_pool::WlShmPool;
use wayland_client::{
    Connection, Dispatch, QueueHandle,
    protocol::wl_buffer::{self, WlBuffer},
};

pub struct Buffer {
    pub buffer: WlBuffer,
    pub in_use: bool,
    pub data: *mut u8,
}

impl WaylandState {
    /// Safe wrapper for `mmap`
    ///
    /// The return value is a pointer to the memory-mapped region
    pub fn map_file(
        &self,
        len: usize,
        prot: i32,
        flags: i32,
        fd: RawFd,
    ) -> Result<*mut c_void, Box<dyn std::error::Error>> {
        let ptr = unsafe { mmap(std::ptr::null_mut(), len as usize, prot, flags, fd, 0) };

        if ptr == nix::libc::MAP_FAILED {
            return Err("mmap failed".into());
        }

        Ok(ptr)
    }

    /// Create an in-memory file with a given size, and optional name
    ///
    /// Returns an `OwnedFd` for the created file
    fn create_memfd(
        &self,
        size: usize,
        name: Option<&str>,
    ) -> Result<OwnedFd, Box<dyn std::error::Error>> {
        let memfd_name = name.unwrap_or("tlockr_pool");
        let fd = memfd_create(memfd_name, MFdFlags::empty())?;
        unsafe { ftruncate(fd.as_raw_fd(), size as i64) };
        Ok(fd)
    }

    /// Allocate a new display buffer from the pool
    ///
    /// This function allocates a new buffer in ARGB8888 format, and adds it to the internal buffer store.
    fn allocate_buffer(
        &mut self,
        pool: &WlShmPool,
        qh: &QueueHandle<Self>,
        data_ptr: *mut c_void,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let (width, height, stride, _) = self.calculate_buffer_dimensions()?;

        let index = self.buffers.as_ref().iter().count() as i32;

        let offset = index * stride * height;
        let ptr = unsafe { (data_ptr as *mut u8).offset(offset as isize) };

        let buffer = pool.create_buffer(
            offset,
            width,
            height,
            stride,
            wl_shm::Format::Argb8888,
            &qh,
            index,
        );

        if let Some(buffers) = &mut self.buffers {
            buffers.push(Buffer {
                buffer,
                in_use: false,
                data: ptr,
            });
        }

        Ok(())
    }

    /// This function returns the dimensions of a display buffer in the format `(width, height, stride, size)`.
    fn calculate_buffer_dimensions(
        &self,
    ) -> Result<(i32, i32, i32, i32), Box<dyn std::error::Error>> {
        if self.width < 0 || self.height < 0 {
            return Err("Invalid width or height".into());
        }

        let stride = self.width * 4;

        Ok((self.width, self.height, stride, self.height * stride))
    }

    /// Create a new pool and allocate `n` buffers in it
    ///
    /// The allocated buffers are stored in the buffer store
    pub fn allocate_buffers(
        &mut self,
        event_queue: &EventQueue<Self>,
        n: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let (_, _, _, buffer_size) = self.calculate_buffer_dimensions()?;

        let size = buffer_size * n;

        let qh = event_queue.handle();

        let shm = self
            .shm
            .as_ref()
            .ok_or::<Box<dyn std::error::Error>>("shm is None".into())?;

        let fd = self.create_memfd(size as usize, Some("tlockr_pool"))?;

        let data_ptr = self.map_file(
            size as usize,
            PROT_READ | PROT_WRITE,
            MAP_SHARED,
            fd.as_raw_fd(),
        )?;

        let pool = shm.create_pool(fd.as_fd(), size as i32, &qh, ());

        for _ in 0..n {
            self.allocate_buffer(&pool, &qh, data_ptr)?;
        }

        println!("Allocated {} buffers: {} bytes", n, size);

        Ok(())
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
                // When a buffer is released, mark it as available for use in the store
                if let Some(buffers) = state.buffers.as_mut() {
                    buffers[*data as usize].in_use = false;
                }
            }
            _ => {}
        }
    }
}
