use crate::wayland::communication::{param::EventParam, serial::next_serial};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum EventType {
    Wayland = 0,
    Renderer = 1,
}

impl TryFrom<u32> for EventType {
    type Error = &'static str;

    fn try_from(tag: u32) -> Result<Self, Self::Error> {
        match tag {
            0 => Ok(EventType::Wayland),
            1 => Ok(EventType::Renderer),
            _ => Err("Invalid EventType tag"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Event {
    pub serial: u32,
    pub event_type: EventType,
    pub param_1: EventParam,
    pub param_2: EventParam,
}

impl Event {
    pub fn new(event_type: EventType, param_1: EventParam, param_2: EventParam) -> Self {
        Self {
            serial: next_serial(),
            event_type,
            param_1,
            param_2,
        }
    }

    pub unsafe fn from_ptr(ptr: *const Event) -> Self {
        unsafe { std::ptr::read(ptr) }
    }

    pub unsafe fn from_mut_ptr(ptr: *mut Event) -> Self {
        unsafe { std::ptr::read(ptr) }
    }
}
