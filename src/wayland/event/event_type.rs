/// The type of event that is being handled
///
/// This enum is C-compatible.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u64)]
pub enum EventType {
    Wayland = 1,
    Renderer = 2,
}

impl TryFrom<u32> for EventType {
    type Error = &'static str;

    /// Create an `EventType` from an event tag value
    fn try_from(tag: u32) -> Result<Self, Self::Error> {
        match tag {
            1 => Ok(EventType::Wayland),
            2 => Ok(EventType::Renderer),
            _ => Err("Invalid EventType tag"),
        }
    }
}
