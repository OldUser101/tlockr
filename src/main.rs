pub mod wayland;

use wayland::state::LockState;

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

    lock_state.run_event_loop(&mut event_queue)?;

    lock_state.destroy_renderer();

    Ok(())
}
