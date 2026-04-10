use rlfw::core::app::App;
use rlfw::core::hints::*;
use rlfw::platform;

pub fn main() {
    let app = platform::init().unwrap();

    let hints = Hints::new().gl_version(3, 3).gl_profile(GlProfile::Core);

    let mut window = app.create_window(800, 600, "window", Some(hints)).unwrap();

    while !window.should_close() {
        window.poll_events();
        window.swap_buffers();
    }
}
