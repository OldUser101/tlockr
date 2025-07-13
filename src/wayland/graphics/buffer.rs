// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025, Nathan Gill

use crate::wayland::state::WaylandState;
use nix::libc::{MAP_SHARED, PROT_READ, PROT_WRITE, ftruncate, mmap};
use nix::sys::memfd::{MFdFlags, memfd_create};
use std::os::fd::{AsFd, AsRawFd, OwnedFd};
use wayland_client::EventQueue;
use wayland_client::protocol::wl_shm;
use wayland_client::{
    Connection, Dispatch, QueueHandle,
    protocol::wl_buffer::{self, WlBuffer},
};

pub struct Buffer {
    pub buffer: WlBuffer,
    pub in_use: bool,
    pub data: *mut u8,
}

fn create_memfd(size: usize) -> Result<OwnedFd, Box<dyn std::error::Error>> {
    let fd = memfd_create("tlockr_pool", MFdFlags::empty())?;
    unsafe { ftruncate(fd.as_raw_fd(), size as i64) };
    Ok(fd)
}

impl WaylandState {
    pub fn allocate_buffers(
        &mut self,
        event_queue: &EventQueue<Self>,
        n: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if self.shm.is_none() {
            return Err(Box::<dyn std::error::Error>::from("shm is None"));
        }

        if self.width < 0 || self.height < 0 {
            return Err(Box::<dyn std::error::Error>::from(
                "Invalid width or height",
            ));
        }

        let stride = self.width * 4;
        let size = self.height * stride * n;

        let qh = event_queue.handle();

        let shm = self.shm.as_ref().unwrap();
        let fd = create_memfd(size as usize)?;

        let data_ptr = unsafe {
            mmap(
                std::ptr::null_mut(),
                size as usize,
                PROT_READ | PROT_WRITE,
                MAP_SHARED,
                fd.as_raw_fd(),
                0,
            )
        };

        if data_ptr == nix::libc::MAP_FAILED {
            return Err(Box::<dyn std::error::Error>::from("mmap failed"));
        }

        let pool = shm.create_pool(fd.as_fd(), size as i32, &qh, ());

        for i in 0..n {
            let buffer = pool.create_buffer(
                (i * stride * self.height) as i32,
                self.width,
                self.height,
                stride,
                wl_shm::Format::Argb8888,
                &qh,
                i,
            );

            let buffer_offset = (i * stride * self.height) as isize;
            let buffer_data = unsafe { (data_ptr as *mut u8).offset(buffer_offset) };

            self.buffers.as_mut().unwrap().push(Buffer {
                buffer: buffer,
                in_use: false,
                data: buffer_data,
            });
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
                if let Some(buffers) = state.buffers.as_mut() {
                    buffers[*data as usize].in_use = false;
                }
            }
            _ => {}
        }
    }
}
