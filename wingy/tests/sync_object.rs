#[allow(dead_code)]
fn needs_sync<T: Sync + ?Sized>() {}

#[test]
fn event_loop_proxy_send() {
    needs_sync::<wingy::event_loop::EventLoopProxy>();
}

#[test]
fn window_sync() {
    needs_sync::<dyn wingy::window::Window>();
}

#[test]
fn window_builder_sync() {
    needs_sync::<wingy::window::WindowAttributes>();
}

#[test]
fn custom_cursor_sync() {
    needs_sync::<wingy::cursor::CustomCursorSource>();
    needs_sync::<wingy::cursor::CustomCursor>();
}

#[test]
fn owned_display_handle_sync() {
    needs_sync::<wingy::event_loop::OwnedDisplayHandle>();
}
