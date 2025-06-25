mod interface;
mod lock;
mod state;

use state::LockState;

use crate::lock::State;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Initializing Wayland interfaces...");

    let mut lock_state = LockState::new();

    let mut event_queue = lock_state.initialize()?;

    lock_state.roundtrip(&mut event_queue)?;

    println!("Wayland interfaces initialized successfully.");

    lock_state.lock(&event_queue)?;

    while lock_state.state != State::Unlocked {
        lock_state.dispatch_event(&mut event_queue)?;
    }

    Ok(())
}
