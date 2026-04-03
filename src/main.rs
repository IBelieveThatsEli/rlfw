use rlfw::core::app::App;
use rlfw::platform;

pub fn main() {
    let app = platform::init().unwrap();
    let mut window = app.create_window(800, 600, "window").unwrap();

    while !window.should_close() {
        window.poll_events();
    }
}
