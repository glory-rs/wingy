//! Handling of the wp-viewporter.

use sctk::globals::GlobalData;
use sctk::reexports::client::globals::{BindError, GlobalList};
use sctk::reexports::client::protocol::wl_surface::WlSurface;
use sctk::reexports::client::{Connection, Dispatch, Proxy, QueueHandle, delegate_dispatch};
use sctk::reexports::protocols::wp::viewporter::client::wp_viewport::WpViewport;
use sctk::reexports::protocols::wp::viewporter::client::wp_viewporter::WpViewporter;

use crate::state::WingyState;

/// Viewporter.
#[derive(Debug)]
pub struct ViewporterState {
    viewporter: WpViewporter,
}

impl ViewporterState {
    /// Create new viewporter.
    pub fn new(
        globals: &GlobalList,
        queue_handle: &QueueHandle<WingyState>,
    ) -> Result<Self, BindError> {
        let viewporter = globals.bind(queue_handle, 1..=1, GlobalData)?;
        Ok(Self { viewporter })
    }

    /// Get the viewport for the given object.
    pub fn get_viewport(
        &self,
        surface: &WlSurface,
        queue_handle: &QueueHandle<WingyState>,
    ) -> WpViewport {
        self.viewporter.get_viewport(surface, queue_handle, GlobalData)
    }
}

impl Dispatch<WpViewporter, GlobalData, WingyState> for ViewporterState {
    fn event(
        _: &mut WingyState,
        _: &WpViewporter,
        _: <WpViewporter as Proxy>::Event,
        _: &GlobalData,
        _: &Connection,
        _: &QueueHandle<WingyState>,
    ) {
        // No events.
    }
}
impl Dispatch<WpViewport, GlobalData, WingyState> for ViewporterState {
    fn event(
        _: &mut WingyState,
        _: &WpViewport,
        _: <WpViewport as Proxy>::Event,
        _: &GlobalData,
        _: &Connection,
        _: &QueueHandle<WingyState>,
    ) {
        // No events.
    }
}

delegate_dispatch!(WingyState: [WpViewporter: GlobalData] => ViewporterState);
delegate_dispatch!(WingyState: [WpViewport: GlobalData] => ViewporterState);
