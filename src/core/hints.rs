#[derive(Debug, Clone, Default)]
pub struct Hints {
    pub gl_major_version: Option<i32>,
    pub gl_minor_version: Option<i32>,
    pub gl_profile: Option<GlProfile>,
    // pub resizable: Option<bool>,
    // pub visible: Option<bool>,
    // pub decorated: Option<bool>,
    // Add more hints as needed
}

#[derive(Debug, Clone, Copy)]
pub enum GlProfile {
    Core,
    Compat,
    Any,
}

impl Hints {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn gl_version(mut self, major: i32, minor: i32) -> Self {
        self.gl_major_version = Some(major);
        self.gl_minor_version = Some(minor);
        self
    }

    pub fn gl_profile(mut self, profile: GlProfile) -> Self {
        self.gl_profile = Some(profile);
        self
    }
}
