#[allow(dead_code)]
fn needs_send<T: Send + ?Sized>() {}

#[test]
fn event_loop_proxy_send() {
    needs_send::<wingy::event_loop::EventLoopProxy>();
}

#[test]
fn window_send() {
    needs_send::<dyn wingy::window::Window>();
}

#[test]
fn window_builder_send() {
    needs_send::<wingy::window::WindowAttributes>();
}

#[test]
fn ids_send() {
    needs_send::<wingy::window::WindowId>();
    needs_send::<wingy::event::DeviceId>();
    needs_send::<wingy::monitor::MonitorHandle>();
}

#[test]
fn custom_cursor_send() {
    needs_send::<wingy::cursor::CustomCursorSource>();
    needs_send::<wingy::cursor::CustomCursor>();
}

#[test]
fn owned_display_handle_send() {
    needs_send::<wingy::event_loop::OwnedDisplayHandle>();
}
