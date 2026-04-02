use crate::core::app::App as IApp;
use crate::core::window::Window as IWindow;

use std::{
    ffi::{c_char, c_int, c_long, c_uint, c_ulong},
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

const KEY_PRESS_MASK: c_long = 1 << 0;
const KEY_RELEASE_MASK: c_long = 1 << 1;
const BUTTON_PRESS_MASK: c_long = 1 << 2;
const BUTTON_RELEASE_MASK: c_long = 1 << 3;
// const ENTER_WINDOW_MASK: c_long = 1 << 4;
// const LEAVE_WINDOW_MASK: c_long = 1 << 5;
// const POINTER_MOTION_MASK: c_long = 1 << 6;
const EXPOSURE_MASK: c_long = 1 << 15;
const STRUCTURE_NOTIFY_MASK: c_long = 1 << 17;

const BASIC_EVENT_MASK: c_long = KEY_PRESS_MASK
    | KEY_RELEASE_MASK
    | BUTTON_PRESS_MASK
    | BUTTON_RELEASE_MASK
    | EXPOSURE_MASK
    | STRUCTURE_NOTIFY_MASK;

#[allow(dead_code)]
unsafe extern "C" {
    fn XOpenDisplay(display_name: *const c_char) -> Display;
    fn XCloseDisplay(display: *mut Display) -> c_int;

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
    fn XRootWindow(display: *mut Display, screen: c_int) -> c_ulong;

    fn XMapWindow(display: *mut Display, window: c_ulong) -> c_int;

    fn XSelectInput(display: *mut Display, window: c_ulong, event_mask: c_long) -> c_int;

    fn XNextEvent(display: *mut Display, event_out: *mut XEvent) -> c_int;

    fn XBlackPixel(display: *mut Display, screen_number: c_int) -> c_ulong;
    fn XWhitePixel(display: *mut Display, screen_number: c_int) -> c_ulong;
}

#[allow(dead_code)]
#[derive(Default)]
pub struct Backend {
    display: *mut Display,
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
    window: c_ulong,
}

impl IApp for App {
    fn create_window(
        &self,
        width: u32,
        height: u32,
        _title: &str,
    ) -> Result<Box<dyn IWindow>, String> {
        unsafe {
            let window = XCreateSimpleWindow(
                self.backend.display,
                XRootWindow(self.backend.display, self.backend.screen),
                0,
                0,
                width,
                height,
                1,
                XBlackPixel(self.backend.display, self.backend.screen),
                XWhitePixel(self.backend.display, self.backend.screen),
            );

            XSelectInput(self.backend.display, window, BASIC_EVENT_MASK);

            XMapWindow(self.backend.display, window);

            let out_window = Box::new(Window {
                backend: self.backend.clone(),
                window: window,
            });

            Ok(out_window)
        }
    }
}

impl App {
    pub fn init() -> Result<Self, String> {
        unsafe {
            let mut display = XOpenDisplay(null());

            let screen = XDefaultScreen(&mut display);

            Ok(App {
                backend: Arc::new({
                    Backend {
                        display: &mut display,
                        screen: screen,
                        event: XEvent::default(),
                    }
                }),
            })
        }
    }
}

impl Drop for App {
    fn drop(&mut self) {
        unsafe {
            XCloseDisplay(self.backend.display);
        }
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
