unsafe extern "C" {
    pub fn render_single_frame(
        qml_path: *const ::std::os::raw::c_char,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        buffer: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
