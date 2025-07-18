use std::os::raw::c_void;

#[derive(Debug, Clone, Copy)]
pub struct EventParam(u64);

impl EventParam {
    pub fn new<T: Into<EventParam>>(value: T) -> Self {
        value.into()
    }

    pub fn as_<T: From<EventParam>>(self) -> T {
        T::from(self)
    }

    pub fn raw(self) -> u64 {
        self.0
    }
}

impl From<EventParam> for *mut c_void {
    fn from(param: EventParam) -> Self {
        param.0 as *mut c_void
    }
}

impl From<*mut c_void> for EventParam {
    fn from(ptr: *mut c_void) -> Self {
        EventParam(ptr as u64)
    }
}

impl From<*const c_void> for EventParam {
    fn from(ptr: *const c_void) -> Self {
        EventParam(ptr as u64)
    }
}
