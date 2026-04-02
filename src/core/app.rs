use crate::core::window::Window;

#[allow(dead_code)]
pub trait App {
    fn create_window(
        &self,
        width: u32,
        height: u32,
        title: &str,
    ) -> Result<Box<dyn Window>, String>;
}
