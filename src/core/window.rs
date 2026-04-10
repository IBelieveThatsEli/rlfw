#[allow(dead_code)]
pub trait Window {
    fn poll_events(&mut self);
    fn should_close(&self) -> bool;

    fn swap_buffers(&self);
}
