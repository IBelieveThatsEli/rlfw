#[allow(dead_code)]
pub trait Window {
    fn poll_events(&self);
    fn should_close(&self) -> bool;
}
