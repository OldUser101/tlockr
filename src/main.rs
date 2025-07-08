mod buffer;
mod ffi;
mod interface;
mod keyboard;
mod lock;
mod state;

use state::LockState;

use crate::lock::State;

use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: tlockr <qml path>");
        return Err("Invalid number of arguments".into());
    }

    println!("Initializing Wayland interfaces...");

    let mut lock_state = LockState::new(args[1].clone());

    let mut event_queue = lock_state.initialize()?;

    lock_state.roundtrip(&mut event_queue)?;

    println!("Wayland interfaces initialized successfully.");

    while lock_state.state != State::Unlocked {
        lock_state.dispatch_event(&mut event_queue)?;

        if lock_state.state == State::Ready {
            lock_state.allocate_buffers(&event_queue, 2)?;
            lock_state.lock(&event_queue)?;
        }
    }

    Ok(())
}
