pub use std::ffi::{CString, c_char, c_int, c_long, c_uint, c_ulong, c_void};

pub type GLXContext = *mut c_void;
pub type GLXFBConfig = *mut c_void;

pub const GLX_RGBA: c_int = 4;
pub const GLX_DOUBLEBUFFER: c_int = 5;
pub const GLX_RED_SIZE: c_int = 8;
pub const GLX_GREEN_SIZE: c_int = 9;
pub const GLX_BLUE_SIZE: c_int = 10;
pub const GLX_DEPTH_SIZE: c_int = 12;
pub const GLX_CONTEXT_MAJOR_VERSION_ARB: c_int = 0x2091;
pub const GLX_CONTEXT_MINOR_VERSION_ARB: c_int = 0x2092;
pub const GLX_CONTEXT_PROFILE_MASK_ARB: c_int = 0x9126;
pub const GLX_CONTEXT_CORE_PROFILE_BIT_ARB: c_int = 0x00000001;
pub const GLX_X_RENDERABLE: c_int = 0x8012;
pub const GLX_DRAWABLE_TYPE: c_int = 0x8010;
pub const GLX_RENDER_TYPE: c_int = 0x8011;
pub const GLX_X_VISUAL_TYPE: c_int = 0x22;
pub const GLX_TRUE_COLOR: c_int = 0x8002;
pub const GLX_WINDOW_BIT: c_int = 0x00000001;
pub const GLX_RGBA_BIT: c_int = 0x00000001;
pub const GLX_ALPHA_SIZE: c_int = 11;
pub const GLX_STENCIL_SIZE: c_int = 13;
pub const GLX_CONTEXT_COMPATIBILITY_PROFILE_BIT_ARB: c_int = 0x00000002;

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

unsafe extern "C" {
    pub fn glXCreateContext(
        display: *mut Display,
        visual_info: *mut XVisualInfo,
        share_list: GLXContext,
        direct: c_int,
    ) -> GLXContext;

    pub fn glXMakeCurrent(display: *mut Display, drawable: c_ulong, context: GLXContext) -> c_int;

    pub fn glXDestroyContext(display: *mut Display, context: GLXContext);
    pub fn glXCreateContextAttribsARB(
        display: *mut Display,
        config: GLXFBConfig,
        share_context: GLXContext,
        direct: c_int,
        attrib_list: *const c_int,
    ) -> GLXContext;

    pub fn glXChooseFBConfig(
        display: *mut Display,
        screen: c_int,
        attrib_list: *const c_int,
        nelements: *mut c_int,
    ) -> *mut GLXFBConfig;

    pub fn glXGetProcAddress(proc_name: *const c_char) -> *mut c_void;

    pub fn glXGetProcAddressARB(proc_name: *const c_char) -> *mut c_void;

    pub fn glXSwapBuffers(display: *mut Display, drawable: c_ulong);

    pub fn XFree(data: *mut c_void) -> c_int;

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
    // pub fn XFlush(display: *mut Display);
    pub fn XSync(display: *mut Display, discard: c_int) -> c_int;

    pub fn XSelectInput(display: *mut Display, window: c_ulong, event_mask: c_long) -> c_int;

    pub fn XPending(display: *mut Display) -> c_int;
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

#[repr(C)]
pub struct XVisualInfo {
    pub visual: *mut c_void,
    pub visualid: c_ulong,
    pub screen: c_int,
    pub depth: c_int,
    pub class: c_int,
    pub red_mask: c_ulong,
    pub green_mask: c_ulong,
    pub blue_mask: c_ulong,
    pub colormap_size: c_int,
    pub bits_per_rgb: c_int,
}
