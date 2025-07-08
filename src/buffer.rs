use std::os::fd::{AsFd, AsRawFd, OwnedFd};

use nix::libc::{MAP_SHARED, PROT_READ, PROT_WRITE, ftruncate, mmap};
use nix::sys::memfd::{MFdFlags, memfd_create};
use wayland_client::protocol::wl_buffer::{self, WlBuffer};
use wayland_client::protocol::wl_shm;
use wayland_client::{Connection, Dispatch, EventQueue, QueueHandle};

use crate::state::LockState;

pub struct Buffer {
    pub buffer: WlBuffer,
    pub in_use: bool,
    pub data: *mut u8,
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
        n: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if self.interfaces.shm.is_none() {
            return Err(Box::<dyn std::error::Error>::from("shm is None"));
        }

        if self.interfaces.width < 0 || self.interfaces.height < 0 {
            return Err(Box::<dyn std::error::Error>::from(
                "Invalid width or height",
            ));
        }

        let stride = self.interfaces.width * 4;
        let size = self.interfaces.height * stride * n;

        let qh = event_queue.handle();

        let shm = self.interfaces.shm.as_ref().unwrap();
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
                (i * stride * self.interfaces.height) as i32,
                self.interfaces.width,
                self.interfaces.height,
                stride,
                wl_shm::Format::Argb8888,
                &qh,
                i,
            );

            let buffer_offset = (i * stride * self.interfaces.height) as isize;
            let buffer_data = unsafe { (data_ptr as *mut u8).offset(buffer_offset) };

            self.interfaces.buffers.as_mut().unwrap().push(Buffer {
                buffer: buffer,
                in_use: false,
                data: buffer_data,
            });
        }

        println!("Allocated {} buffers: {} bytes", n, size);

        Ok(())
    }
}
