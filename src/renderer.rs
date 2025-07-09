use crate::{
    ffi::{QmlRenderer, cleanup_renderer, initialize_renderer},
    state::LockState,
};

use std::ffi::CString;

pub struct QmlRendererInterface {
    pub renderer: *mut QmlRenderer,
    pub qml_path: Option<CString>,
}

impl QmlRendererInterface {
    pub fn new() -> Self {
        Self {
            renderer: std::ptr::null_mut(),
            qml_path: None,
        }
    }
}

impl LockState {
    pub fn initialize_renderer(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let qml_path = CString::new(self.qml_path.as_str()).unwrap();

        let renderer = unsafe {
            initialize_renderer(
                self.interfaces.width,
                self.interfaces.height,
                qml_path.as_ptr(),
            )
        };

        if renderer != std::ptr::null_mut() {
            self.renderer.renderer = renderer;
            self.renderer.qml_path = Some(qml_path);
        } else {
            return Err("QML renderer initialization failed".into());
        }

        Ok(())
    }

    pub fn destroy_renderer(&mut self) {
        if self.renderer.renderer != std::ptr::null_mut() {
            unsafe {
                cleanup_renderer(self.renderer.renderer);
                self.renderer.renderer = std::ptr::null_mut();
                self.renderer.qml_path = None;
            }
        }
    }
}
