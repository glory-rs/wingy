use sctk::globals::GlobalData;
use sctk::reexports::client::globals::{BindError, GlobalList};
use sctk::reexports::client::protocol::wl_surface::WlSurface;
use sctk::reexports::client::{Connection, Dispatch, Proxy, QueueHandle, delegate_dispatch};
use wayland_protocols::ext::background_effect::v1::client::ext_background_effect_manager_v1::ExtBackgroundEffectManagerV1;
use wayland_protocols::ext::background_effect::v1::client::ext_background_effect_surface_v1::ExtBackgroundEffectSurfaceV1;

use crate::state::WingyState;

#[derive(Debug, Clone)]
pub struct ExtBackgroundEffectManager {
    manager: ExtBackgroundEffectManagerV1,
}

impl ExtBackgroundEffectManager {
    pub fn new(
        globals: &GlobalList,
        queue_handle: &QueueHandle<WingyState>,
    ) -> Result<Self, BindError> {
        let manager = globals.bind(queue_handle, 1..=1, GlobalData)?;
        Ok(Self { manager })
    }

    pub fn blur(
        &mut self,
        surface: &WlSurface,
        queue_handle: &QueueHandle<WingyState>,
    ) -> ExtBackgroundEffectSurfaceV1 {
        self.manager.get_background_effect(surface, queue_handle, ())
    }
}

impl Dispatch<ExtBackgroundEffectManagerV1, GlobalData, WingyState> for ExtBackgroundEffectManager {
    fn event(
        _: &mut WingyState,
        _: &ExtBackgroundEffectManagerV1,
        _: <ExtBackgroundEffectManagerV1 as Proxy>::Event,
        _: &GlobalData,
        _: &Connection,
        _: &QueueHandle<WingyState>,
    ) {
    }
}

impl Dispatch<ExtBackgroundEffectSurfaceV1, (), WingyState> for ExtBackgroundEffectManager {
    fn event(
        _: &mut WingyState,
        _: &ExtBackgroundEffectSurfaceV1,
        _: <ExtBackgroundEffectSurfaceV1 as Proxy>::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<WingyState>,
    ) {
        // There is no event
    }
}

delegate_dispatch!(WingyState: [ExtBackgroundEffectManagerV1: GlobalData] => ExtBackgroundEffectManager);
delegate_dispatch!(WingyState: [ExtBackgroundEffectSurfaceV1: ()] => ExtBackgroundEffectManager);
