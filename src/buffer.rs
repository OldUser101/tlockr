use std::os::fd::{AsFd, AsRawFd, OwnedFd};

use nix::libc::ftruncate;
use nix::sys::memfd::{MFdFlags, memfd_create};
use wayland_client::protocol::wl_buffer::{self, WlBuffer};
use wayland_client::protocol::wl_shm;
use wayland_client::{Connection, Dispatch, EventQueue, QueueHandle};

use crate::state::LockState;

pub struct Buffer {
    pub buffer: WlBuffer,
    pub in_use: bool,
}

impl Dispatch<WlBuffer, i32> for LockState {
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
                if let Some(buffers) = state.interfaces.buffers.as_mut() {
                    buffers[*data as usize].in_use = false;
                }
            }
            _ => {}
        }
    }
}

fn create_memfd(size: usize) -> Result<OwnedFd, Box<dyn std::error::Error>> {
    let fd = memfd_create("tlockr_pool", MFdFlags::empty())?;
    unsafe { ftruncate(fd.as_raw_fd(), size as i64) };
    Ok(fd)
}

impl LockState {
    pub fn allocate_buffers(
        &mut self,
        event_queue: &EventQueue<LockState>,
        width: i32,
        height: i32,
        n: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if self.interfaces.shm.is_none() {
            return Err(Box::<dyn std::error::Error>::from("shm is None"));
        }

        let stride = width * 4;
        let size = height * stride * n;

        let qh = event_queue.handle();

        let shm = self.interfaces.shm.as_ref().unwrap();
        let fd = create_memfd(size as usize)?;

        let pool = shm.create_pool(fd.as_fd(), size as i32, &qh, ());

        for i in 0..n {
            let buffer = pool.create_buffer(
                (i * stride * height) as i32,
                width,
                height,
                stride,
                wl_shm::Format::Argb8888,
                &qh,
                i,
            );

            self.interfaces.buffers.as_mut().unwrap().push(Buffer {
                buffer: buffer,
                in_use: false,
            });
        }

        println!("Allocated {} buffers: {} bytes", n, size);

        Ok(())
    }
}
