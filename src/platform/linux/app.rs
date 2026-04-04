use super::{backend::*, ffi::*, window::Window};
use crate::core::app::App as IApp;
use crate::core::window::Window as IWindow;

use std::ptr::null;

#[allow(dead_code)]
#[derive(Default)]
pub struct App {
    backend: SharedBackend,
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
