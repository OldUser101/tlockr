use crate::{interface::WaylandInterfaces, lock::State};

use wayland_client::{
    Connection, Dispatch, EventQueue, QueueHandle,
    protocol::{
        wl_compositor::WlCompositor,
        wl_display::WlDisplay,
        wl_output::WlOutput,
        wl_registry::{Event as RegistryEvent, WlRegistry},
        wl_seat::WlSeat,
        wl_shm::WlShm,
        wl_shm_pool::WlShmPool,
        wl_surface::WlSurface,
    },
};
use wayland_protocols::{
    ext::session_lock::v1::client::ext_session_lock_manager_v1::ExtSessionLockManagerV1,
    wp::viewporter::client::{wp_viewport::WpViewport, wp_viewporter::WpViewporter},
};

pub struct LockState {
    pub interfaces: WaylandInterfaces,
    pub state: State,
}

impl LockState {
    pub fn new() -> Self {
        Self {
            interfaces: WaylandInterfaces::new(),
            state: State::Ready,
        }
    }

    pub fn initialize(&mut self) -> Result<EventQueue<Self>, Box<dyn std::error::Error>> {
        let event_queue = self.interfaces.create_and_bind()?;
        self.state = State::Initialized;

        Ok(event_queue)
    }

    pub fn roundtrip(
        &mut self,
        event_queue: &mut EventQueue<Self>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        event_queue.roundtrip(self)?;
        Ok(())
    }

    pub fn dispatch_event(
        &mut self,
        event_queue: &mut EventQueue<Self>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        event_queue.blocking_dispatch(self)?;
        Ok(())
    }
}

macro_rules! empty_dispatch {
    ($state:ty, $($proxy:ty),*) => {
        $(
            impl Dispatch<$proxy, ()> for $state {
                fn event(
                    _state: &mut Self,
                    _proxy: &$proxy,
                    _event: <$proxy as wayland_client::Proxy>::Event,
                    _data: &(),
                    _conn: &Connection,
                    _qh: &QueueHandle<Self>,
                ) {}
            }
        )*
    };
}

empty_dispatch! {
    LockState,
    WlDisplay,
    WlOutput,
    WlCompositor,
    WpViewporter,
    ExtSessionLockManagerV1,
    WlSurface,
    WlShm,
    WlShmPool,
    WpViewport
}

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
