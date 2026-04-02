#[cfg(target_os = "linux")]
pub mod linux;
#[cfg(target_os = "linux")]
use linux as platform;

pub use platform::App;

#[allow(dead_code)]
pub fn init() -> bool {
    App::init().is_ok()
}
