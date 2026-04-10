pub mod core;
pub mod platform;

#[cfg(test)]
mod tests {
    use crate::core::app::App;
    use crate::platform;

    #[test]
    fn test_app() {
        let app = platform::init().unwrap();
        let window = app.create_window(800, 600, "Window", None).unwrap();

        assert_eq!(window.should_close(), false);
    }
}
