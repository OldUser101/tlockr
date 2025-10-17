// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025, Nathan Gill

/*
    fallback.rs:
        Fallback renderer in case QML fails
*/

use crate::buffer::Buffer;

use bitmap_font::{tamzen::FONT_10x20_BOLD, TextStyle};
use embedded_graphics::{
    pixelcolor::BinaryColor, prelude::*, text::Text, Pixel
};
use tracing::info;
use wayland_client::protocol::wl_surface::WlSurface;
use wayland_protocols::wp::viewporter::client::wp_viewport::WpViewport;

const HEADER_STRING: &str = "** TLOCKR FALLBACK MODE **";

pub struct FallbackRenderer {
    surface: *const WlSurface,
    viewport: *const WpViewport,
    buffer: *mut Buffer,
    width: i32,
    height: i32,
}

impl FallbackRenderer {
    pub fn new(
        surface: *const WlSurface,
        viewport: *const WpViewport,
        buffer: *mut Buffer,
        width: i32,
        height: i32,
    ) -> Self {
        Self {
            surface,
            viewport,
            buffer,
            width,
            height,
        }
    }

    /// Initialize the fallback renderer
    pub fn initialize(&mut self) {
        self.clear_background();
        self.draw_header();
        self.refresh();

        info!("Initialized fallback renderer");
    }

    /// Set the colour of an individual pixel
    fn set_pixel(&mut self, buffer: &mut [u8], x: i32, y: i32, color: &[u8; 4]) {
        let idx = (y * self.width * 4 + x * 4) as usize;
        buffer[idx..idx + 4].copy_from_slice(color);
    }

    /// Clear buffer background
    fn clear_background(&mut self) {
        let buf_len = self.width * self.height * 4;
        let buffer_ptr = unsafe { (*self.buffer).data };
        let buffer: &mut [u8] =
            unsafe { std::slice::from_raw_parts_mut(buffer_ptr, buf_len as usize) };

        for y in 0..self.height {
            for x in 0..self.width {
                self.set_pixel(buffer, x, y, &[0, 0, 0, 255]);
            }
        }
    }

    /// Draw the fallback header to the buffer
    fn draw_header(&mut self) {
        let head = Text::new(HEADER_STRING, Point::zero(), TextStyle::new(&FONT_10x20_BOLD, BinaryColor::On));
        let _ = head.draw(self);
    }

    /// Refresh the buffer on the screen
    fn refresh(&mut self) {
        let surface = unsafe { &*self.surface };
        let viewport = unsafe { &*self.viewport };
        let buffer = unsafe { &(*self.buffer).buffer };

        surface.attach(Some(buffer), 0, 0);
        surface.damage_buffer(0, 0, i32::MAX, i32::MAX);
        viewport.set_destination(self.width, self.height);
        surface.commit();
    }
}

impl DrawTarget for FallbackRenderer {
    type Color = BinaryColor;
    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        let buf_len = self.width * self.height * 4;
        let buffer_ptr = unsafe { (*self.buffer).data };
        let buffer: &mut [u8] =
            unsafe { std::slice::from_raw_parts_mut(buffer_ptr, buf_len as usize) };

        for Pixel(coord, color) in pixels.into_iter() {
            if let Ok::<(u32, u32), _>((x, y)) = coord.try_into() {
                if x <= self.width as u32 && y <= self.height as u32 {
                    if color.is_on() {
                        self.set_pixel(buffer, x as i32, y as i32, &[255, 255, 255, 255]);
                    } else {
                        self.set_pixel(buffer, x as i32, y as i32, &[0, 0, 0, 255]);
                    }
                }
            }
        }

        Ok(())
    }
}

impl OriginDimensions for FallbackRenderer {
    fn size(&self) -> Size {
        Size::new(self.width as u32, self.height as u32)
    }
}
