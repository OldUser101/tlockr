use crate::wayland::interface::WaylandState;
use wayland_client::EventQueue;

impl WaylandState {
    pub fn lock(
        &mut self,
        event_queue: &EventQueue<Self>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let qh = event_queue.handle();

        if let (Some(compositor), Some(viewporter)) = (&self.compositor, &self.viewporter) {
            let surface = compositor.create_surface(&qh, ());
            let viewport = viewporter.get_viewport(&surface, &qh, ());
            self.surface = Some(surface);
            self.viewport = Some(viewport);
        }

        if let Some(session_lock_manager) = &self.session_lock_manager {
            self.session_lock = Some(session_lock_manager.lock(&qh, ()));
        } else {
            return Err("Failed to lock session.".to_string().into());
        }

        Ok(())
    }
}
