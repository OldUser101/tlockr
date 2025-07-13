/*
    WlRegistry related dispatch handlers
*/

use crate::wayland::state::LockState;
use wayland_client::{
    Connection, Dispatch, QueueHandle,
    protocol::{
        wl_compositor::WlCompositor,
        wl_output::WlOutput,
        wl_registry::{Event as RegistryEvent, WlRegistry},
        wl_seat::WlSeat,
        wl_shm::WlShm,
    },
};
use wayland_protocols::{
    ext::session_lock::v1::client::ext_session_lock_manager_v1::ExtSessionLockManagerV1,
    wp::viewporter::client::wp_viewporter::WpViewporter,
};

impl Dispatch<WlRegistry, ()> for LockState {
    fn event(
        state: &mut Self,
        registry: &WlRegistry,
        event: RegistryEvent,
        _data: &(),
        _conn: &Connection,
        qh: &QueueHandle<Self>,
    ) {
        match event {
            RegistryEvent::Global {
                name,
                interface,
                version,
            } => match interface.as_str() {
                "wl_output" => {
                    let output = registry.bind::<WlOutput, _, _>(name, version, qh, ());
                    state.interfaces.output = Some(output);
                }
                "wl_shm" => {
                    let shm = registry.bind::<WlShm, _, _>(name, version, qh, ());
                    state.interfaces.shm = Some(shm);
                }
                "wl_compositor" => {
                    let compositor = registry.bind::<WlCompositor, _, _>(name, version, qh, ());
                    state.interfaces.compositor = Some(compositor);
                }
                "wl_seat" => {
                    let seat = registry.bind::<WlSeat, _, _>(name, version, qh, ());
                    state.interfaces.seat = Some(seat);
                }
                "wp_viewporter" => {
                    let viewporter = registry.bind::<WpViewporter, _, _>(name, version, qh, ());
                    state.interfaces.viewporter = Some(viewporter);
                }
                "ext_session_lock_manager_v1" => {
                    let session_lock_manager =
                        registry.bind::<ExtSessionLockManagerV1, _, _>(name, version, qh, ());
                    state.interfaces.session_lock_manager = Some(session_lock_manager);
                }
                _ => {}
            },
            _ => {}
        }
    }
}
