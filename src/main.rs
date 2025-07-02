mod ffi;
mod buffer;
mod interface;
mod keyboard;
mod lock;
mod state;

use state::LockState;

use crate::lock::State;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        println!("{}", ffi::test());
    }
    
    return Ok(());
    
    println!("Initializing Wayland interfaces...");

    let mut lock_state = LockState::new();

    let mut event_queue = lock_state.initialize()?;

    lock_state.roundtrip(&mut event_queue)?;

    println!("Wayland interfaces initialized successfully.");

    lock_state.allocate_buffers(&event_queue, 1920, 1200, 2)?;
    lock_state.lock(&event_queue)?;

    while lock_state.state != State::Unlocked {
        lock_state.dispatch_event(&mut event_queue)?;
    }

    Ok(())
}
