use std::os::raw::{c_char, c_int, c_void};

#[repr(C)]
pub struct QmlRenderer {
    _private: [u8; 0],
}

pub type RsGetBufferCallback = unsafe extern "C" fn(user_data: *mut c_void) -> *mut c_void;
pub type RsFrameReadyCallback = unsafe extern "C" fn(user_data: *mut c_void, buffer: *mut c_void);

unsafe extern "C" {
    pub fn initialize_renderer(
        width: c_int,
        height: c_int,
        qml_path: *const c_char,
    ) -> *mut QmlRenderer;

    pub fn start_renderer_app(renderer: *mut QmlRenderer) -> c_int;

    pub fn set_buffer_callbacks(
        renderer: *mut QmlRenderer,
        get_buffer: RsGetBufferCallback,
        frame_ready: RsFrameReadyCallback,
        user_data: *mut c_void,
    );

    pub fn cleanup_renderer(renderer: *mut QmlRenderer);

    pub fn render_single_frame(
        qml_path: *const c_char,
        width: c_int,
        height: c_int,
        buffer: *mut c_void,
    ) -> c_int;
}
