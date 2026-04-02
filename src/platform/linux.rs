use crate::core::app::App as IApp;
use crate::core::window::Window as IWindow;

use std::{
    ffi::{c_char, c_int, c_uint, c_ulong},
    ptr::null,
    sync::Arc,
};

#[repr(C)]
#[derive(Default)]
struct XEvent {
    pad: [u64; 24],
}

#[repr(C)]
#[derive(Default)]
pub struct Display {
    _private: [u8; 0],
}

#[allow(dead_code)]
unsafe extern "C" {
    fn XOpenDisplay(display_name: *const c_char) -> Display;
    fn XDefaultScreen(display: *mut Display) -> c_int;

    fn XCreateSimpleWindow(
        display: *mut Display,
        parent: c_ulong,
        x: c_int,
        y: c_int,
        width: c_uint,
        height: c_uint,
        border_width: c_uint,
        border: c_ulong,
        background: c_ulong,
    ) -> c_ulong;
}

#[allow(dead_code)]
#[derive(Default)]
pub struct Backend {
    display: Display,
    screen: c_int,
    event: XEvent,
}

#[allow(dead_code)]
#[derive(Default)]
pub struct App {
    backend: Arc<Backend>,
}

#[allow(dead_code)]
pub struct Window {
    backend: Arc<Backend>,
}

impl IApp for App {
    fn create_window(
        &self,
        width: u32,
        height: u32,
        title: &str,
    ) -> Result<Box<dyn IWindow>, String> {
        Err("Nigger".into())
    }
}

impl App {
    pub fn init() -> Result<Self, String> {
        let mut out = Self::default();

        unsafe {
            let mut display = XOpenDisplay(null());

            let screen = XDefaultScreen(&mut display);

            out.backend = Arc::new({
                Backend {
                    display: display,
                    screen: screen,
                    event: XEvent::default(),
                }
            });
        }

        Ok(out)
    }
}

impl IWindow for Window {
    fn poll_events(&self) {
        todo!()
    }
    fn should_close(&self) -> bool {
        true
    }
}
