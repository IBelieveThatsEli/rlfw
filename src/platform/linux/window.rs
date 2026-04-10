use std::ptr::null_mut;

use super::{backend::*, ffi::*};
use crate::core::window::Window as IWindow;

pub struct Window {
    pub backend: SharedBackend,
    pub window: c_ulong,
    pub should_close: bool,
}

impl IWindow for Window {
    fn poll_events(&mut self) {
        unsafe {
            let backend = self.backend.lock().unwrap();

            while XPending(backend.display) > 0 {
                let mut xevent: XEvent = std::mem::zeroed();
                XNextEvent(backend.display, &mut xevent);

                match xevent.type_ {
                    CLIENT_MESSAGE => {
                        let xclient = xevent.xclient;

                        if xclient.data[0] == backend.wm_delete as c_long {
                            self.should_close = true;
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    fn should_close(&self) -> bool {
        self.should_close
    }

    fn swap_buffers(&self) {
        unsafe {
            let backend = self.backend.lock().unwrap();
            glXSwapBuffers(backend.display, self.window);
        }
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe {
            let backend = self.backend.lock().unwrap();

            if let Some(context) = backend.gl_context {
                glXMakeCurrent(backend.display, 0, null_mut());
                glXDestroyContext(backend.display, context);
            }
        }
    }
}
