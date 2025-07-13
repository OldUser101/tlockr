use crate::wayland::state::LockState;
use wayland_client::EventQueue;

#[derive(PartialEq)]
pub enum State {
    None,
    Initialized,
    Ready,
    Locked,
    Unlocked,
}

impl LockState {
    pub fn lock(
        &mut self,
        event_queue: &EventQueue<LockState>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let qh = event_queue.handle();

        if let (Some(compositor), Some(viewporter)) =
            (&self.interfaces.compositor, &self.interfaces.viewporter)
        {
            let surface = compositor.create_surface(&qh, ());
            let viewport = viewporter.get_viewport(&surface, &qh, ());
            self.interfaces.surface = Some(surface);
            self.interfaces.viewport = Some(viewport);
        }

        if let Some(session_lock_manager) = &self.interfaces.session_lock_manager {
            self.interfaces.session_lock = Some(session_lock_manager.lock(&qh, ()));
        } else {
            return Err("Failed to lock session.".to_string().into());
        }

        Ok(())
    }
}
