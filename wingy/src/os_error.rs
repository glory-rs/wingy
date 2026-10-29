#[allow(unused_macros)]
macro_rules! os_error {
    ($error:expr) => {{ wingy_core::error::OsError::new(line!(), file!(), $error) }};
}
