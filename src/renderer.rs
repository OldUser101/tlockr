use crate::{ffi::QmlRenderer, state::LockState};

pub struct QmlRendererInterface {
    pub renderer: Option<*mut QmlRenderer>,
}

impl QmlRendererInterface {
    pub fn new() -> Self {
        Self { renderer: None }
    }
}

impl LockState {
    pub fn initialize_renderer(&mut self) {}
}
