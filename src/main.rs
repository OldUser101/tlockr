mod interface;
mod state;

use state::LockState;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Initializing Wayland interfaces...");

    let mut lock_state = LockState::new();

    let mut event_queue = lock_state.initialize()?;

    lock_state.roundtrip(&mut event_queue)?;

    println!("Wayland interfaces initialized successfully.");

    loop {
        lock_state.dispatch_event(&mut event_queue)?;
    }
}
