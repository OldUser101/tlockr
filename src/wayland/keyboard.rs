use xkbcommon_rs::{Keymap, State};

pub struct KeyboardMapping {
    pub file: std::fs::File,
    pub mmap: memmap2::Mmap,
    pub keymap: Option<Keymap>,
    pub state: Option<State>,
}
