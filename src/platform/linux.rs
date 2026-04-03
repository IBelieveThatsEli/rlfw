use crate::core::app::App as IApp;
use crate::core::window::Window as IWindow;

use std::{
    ffi::{CString, c_char, c_int, c_long, c_uint, c_ulong},
    ptr::null,
    sync::{Arc, Mutex},
};

#[repr(C)]
#[derive(Default)]
pub struct XClientMessageEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: c_int,
    pub display: *mut Display,
    pub window: c_ulong,
    pub message_type: c_ulong,
    pub format: c_int,
    pub data: [c_long; 5],
}

#[repr(C)]
#[derive(Default)]
struct XEvent {
    pub type_: c_int,
    pub xclient: XClientMessageEvent,
    pad: [u64; 24],
}

#[repr(C)]
#[derive(Default)]
pub struct Display {
    _private: [u8; 0],
}

const CLIENT_MESSAGE: c_int = 33;
const DESTROY_NOTIFY: c_int = 17;

const KEY_PRESS_MASK: c_long = 1 << 0;
const KEY_RELEASE_MASK: c_long = 1 << 1;
const BUTTON_PRESS_MASK: c_long = 1 << 2;
const BUTTON_RELEASE_MASK: c_long = 1 << 3;
// const ENTER_WINDOW_MASK: c_long = 1 << 4;
const LEAVE_WINDOW_MASK: c_long = 1 << 5;
// const POINTER_MOTION_MASK: c_long = 1 << 6;
const EXPOSURE_MASK: c_long = 1 << 15;
const STRUCTURE_NOTIFY_MASK: c_long = 1 << 17;

const BASIC_EVENT_MASK: c_long = KEY_PRESS_MASK
    | KEY_RELEASE_MASK
    | BUTTON_PRESS_MASK
    | BUTTON_RELEASE_MASK
    | EXPOSURE_MASK
    | STRUCTURE_NOTIFY_MASK
    | LEAVE_WINDOW_MASK;

#[allow(dead_code)]
unsafe extern "C" {
    fn XOpenDisplay(display_name: *const c_char) -> *mut Display;
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
    fn XStoreName(display: *mut Display, window: c_ulong, window_name: *const c_char) -> c_int;

    fn XMapWindow(display: *mut Display, window: c_ulong) -> c_int;
    fn XFlush(display: *mut Display);

    fn XSelectInput(display: *mut Display, window: c_ulong, event_mask: c_long) -> c_int;

    fn XNextEvent(display: *mut Display, event_out: *mut XEvent) -> c_int;

    fn XInternAtom(
        display: *mut Display,
        atom_name: *const c_char,
        only_if_exists: c_int,
    ) -> c_ulong;

    fn XSetWMProtocols(
        display: *mut Display,
        window: c_ulong,
        protocols: *mut c_ulong,
        count: c_int,
    ) -> c_int;

    fn XBlackPixel(display: *mut Display, screen_number: c_int) -> c_ulong;
    fn XWhitePixel(display: *mut Display, screen_number: c_int) -> c_ulong;
}

#[allow(dead_code)]
#[derive(Default)]
pub struct Backend {
    display: *mut Display,
    screen: c_int,
    // event: *mut XEvent,
    wm_delete: c_ulong,
}

#[allow(dead_code)]
#[derive(Default)]
pub struct App {
    backend: Arc<Mutex<Backend>>,
}

#[allow(dead_code)]
pub struct Window {
    backend: Arc<Mutex<Backend>>,
    window: c_ulong,
    should_close: bool,
}

impl IApp for App {
    fn create_window(
        &self,
        width: u32,
        height: u32,
        title: &str,
    ) -> Result<Box<dyn IWindow>, String> {
        unsafe {
            let mut backend = self.backend.lock().unwrap();
            let window = XCreateSimpleWindow(
                backend.display,
                XRootWindow(backend.display, backend.screen),
                0,
                0,
                width,
                height,
                1,
                XBlackPixel(backend.display, backend.screen),
                XWhitePixel(backend.display, backend.screen),
            );

            XSelectInput(backend.display, window, BASIC_EVENT_MASK);
            XMapWindow(backend.display, window);

            let c_string = CString::new(title).expect("CString::new Failed");
            let ptr = c_string.as_ptr();
            XStoreName(backend.display, window, ptr);

            let c_string = CString::new("WM_DELETE_WINDOW").expect("WM_DELETE_WINDOW");
            let ptr = c_string.as_ptr();
            let wm_delete = XInternAtom(backend.display, ptr, 1);
            backend.wm_delete = wm_delete;
            XSetWMProtocols(backend.display, window, &mut backend.wm_delete, 1);

            let out_window = Box::new(Window {
                backend: self.backend.clone(),
                window: window,
                should_close: false,
            });

            Ok(out_window)
        }
    }
}

impl App {
    pub fn init() -> Result<Self, String> {
        unsafe {
            let display = XOpenDisplay(null());

            if display.is_null() {
                return Err("Failed to open X Display".into());
            }

            let screen = XDefaultScreen(display);

            Ok(App {
                backend: Arc::new(Mutex::new({
                    Backend {
                        display: display,
                        screen: screen,
                        // event: &mut XEvent::default(),
                        wm_delete: 0,
                    }
                })),
            })
        }
    }
}

impl Drop for App {
    fn drop(&mut self) {
        unsafe {
            XCloseDisplay(self.backend.lock().unwrap().display);
        }
    }
}

impl IWindow for Window {
    fn poll_events(&mut self) {
        unsafe {
            let backend = self.backend.lock().unwrap();

            let mut xevent: XEvent = std::mem::zeroed();
            XNextEvent(backend.display, &mut xevent);

            println!("Event type: {}", xevent.type_);
            println!("Should close: {}", self.should_close);

            match xevent.type_ {
                CLIENT_MESSAGE => {
                    let xclient = xevent.xclient;

                    println!("data[0]: {}", xclient.data[0]);
                    if xclient.data[0] == backend.wm_delete as c_long {
                        self.should_close = true;
                    }
                }

                DESTROY_NOTIFY => {
                    self.should_close = true;
                }

                _ => {}
            }
        }
    }
    fn should_close(&self) -> bool {
        self.should_close
    }
}
