use super::ffi::*;
pub use std::sync::{Arc, Mutex};

#[allow(dead_code)]
#[derive(Default)]
pub struct Backend {
    pub display: *mut Display,
    pub screen: c_int,
    pub wm_delete: c_ulong,
    pub gl_context: Option<GLXContext>,
}

pub type SharedBackend = Arc<Mutex<Backend>>;
