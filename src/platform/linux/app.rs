use super::{backend::*, ffi::*, window::Window};
use crate::core::app::App as IApp;
use crate::core::hints::{GlProfile, Hints};
use crate::core::window::Window as IWindow;

use std::ptr::{null, null_mut};

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
        hints: Option<Hints>,
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

            XSync(backend.display, 1);

            let c_string = CString::new(title).expect("CString::new Failed");
            let ptr = c_string.as_ptr();
            XStoreName(backend.display, window, ptr);

            let c_string = CString::new("WM_DELETE_WINDOW").expect("WM_DELETE_WINDOW");
            let ptr = c_string.as_ptr();
            let wm_delete = XInternAtom(backend.display, ptr, 1);
            backend.wm_delete = wm_delete;
            XSetWMProtocols(backend.display, window, &mut backend.wm_delete, 1);

            if let Some(hints) = hints {
                if hints.gl_major_version.is_some() || hints.gl_minor_version.is_some() {
                    let major = hints.gl_major_version.unwrap_or(3);
                    let minor = hints.gl_minor_version.unwrap_or(3);

                    let fb_attribs = [
                        GLX_X_RENDERABLE,
                        1,
                        GLX_DRAWABLE_TYPE,
                        GLX_WINDOW_BIT,
                        GLX_RENDER_TYPE,
                        GLX_RGBA_BIT,
                        GLX_X_VISUAL_TYPE,
                        GLX_TRUE_COLOR,
                        GLX_RED_SIZE,
                        8,
                        GLX_GREEN_SIZE,
                        8,
                        GLX_BLUE_SIZE,
                        8,
                        GLX_ALPHA_SIZE,
                        8,
                        GLX_DEPTH_SIZE,
                        24,
                        GLX_STENCIL_SIZE,
                        8,
                        GLX_DOUBLEBUFFER,
                        1,
                        0, // NULL terminator
                    ];

                    let mut fb_count: c_int = 0;
                    let fb_configs = glXChooseFBConfig(
                        backend.display,
                        backend.screen,
                        fb_attribs.as_ptr(),
                        &mut fb_count,
                    );

                    if fb_configs.is_null() || fb_count == 0 {
                        return Err("Failed to get GLX framebuffer config".into());
                    }

                    let fb_config = *fb_configs;

                    let mut context_attribs = vec![
                        GLX_CONTEXT_MAJOR_VERSION_ARB,
                        major,
                        GLX_CONTEXT_MINOR_VERSION_ARB,
                        minor,
                    ];

                    if let Some(profile) = hints.gl_profile {
                        context_attribs.push(GLX_CONTEXT_PROFILE_MASK_ARB);
                        context_attribs.push(match profile {
                            GlProfile::Core => GLX_CONTEXT_CORE_PROFILE_BIT_ARB,
                            GlProfile::Compat => GLX_CONTEXT_COMPATIBILITY_PROFILE_BIT_ARB,
                            GlProfile::Any => 0,
                        });
                    }

                    context_attribs.push(0);

                    let context = glXCreateContextAttribsARB(
                        backend.display,
                        fb_config,
                        null_mut(),
                        1,
                        context_attribs.as_ptr(),
                    );

                    if context.is_null() {
                        return Err(format!(
                            "Failed to create OpenGL v{}.{} context",
                            major, minor
                        ));
                    }

                    glXMakeCurrent(backend.display, window, context);

                    backend.gl_context = Some(context);

                    XFree(fb_configs as *mut _);
                }
            }

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
                        gl_context: None,
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
