pub use std::ffi::{CString, c_char, c_int, c_long, c_uint, c_ulong};

#[repr(C)]
#[derive(Default)]
pub struct Display {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Default, Copy, Clone)]
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
pub union XEvent {
    pub type_: c_int,
    pub xclient: XClientMessageEvent,
    pub pad: [u64; 24],
}

pub const CLIENT_MESSAGE: c_int = 33;
// const DESTROY_NOTIFY: c_int = 17;

const KEY_PRESS_MASK: c_long = 1 << 0;
const KEY_RELEASE_MASK: c_long = 1 << 1;
const BUTTON_PRESS_MASK: c_long = 1 << 2;
const BUTTON_RELEASE_MASK: c_long = 1 << 3;
// const ENTER_WINDOW_MASK: c_long = 1 << 4;
// const LEAVE_WINDOW_MASK: c_long = 1 << 5;
// const POINTER_MOTION_MASK: c_long = 1 << 6;
const EXPOSURE_MASK: c_long = 1 << 15;
const STRUCTURE_NOTIFY_MASK: c_long = 1 << 17;

pub const BASIC_EVENT_MASK: c_long = KEY_PRESS_MASK
    | KEY_RELEASE_MASK
    | BUTTON_PRESS_MASK
    | BUTTON_RELEASE_MASK
    | EXPOSURE_MASK
    | STRUCTURE_NOTIFY_MASK;
#[allow(dead_code)]
unsafe extern "C" {
    pub fn XOpenDisplay(display_name: *const c_char) -> *mut Display;
    pub fn XCloseDisplay(display: *mut Display) -> c_int;

    pub fn XDefaultScreen(display: *mut Display) -> c_int;

    pub fn XCreateSimpleWindow(
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
    pub fn XRootWindow(display: *mut Display, screen: c_int) -> c_ulong;
    pub fn XStoreName(display: *mut Display, window: c_ulong, window_name: *const c_char) -> c_int;

    pub fn XMapWindow(display: *mut Display, window: c_ulong) -> c_int;
    pub fn XFlush(display: *mut Display);

    pub fn XSelectInput(display: *mut Display, window: c_ulong, event_mask: c_long) -> c_int;

    pub fn XNextEvent(display: *mut Display, event_out: *mut XEvent) -> c_int;

    pub fn XInternAtom(
        display: *mut Display,
        atom_name: *const c_char,
        only_if_exists: c_int,
    ) -> c_ulong;

    pub fn XSetWMProtocols(
        display: *mut Display,
        window: c_ulong,
        protocols: *mut c_ulong,
        count: c_int,
    ) -> c_int;

    pub fn XBlackPixel(display: *mut Display, screen_number: c_int) -> c_ulong;
    pub fn XWhitePixel(display: *mut Display, screen_number: c_int) -> c_ulong;
}
