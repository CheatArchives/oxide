use crate::*;

#[derive(Debug, Clone, Copy)]
pub struct Settings {
    pub aimbot: bool,
}

impl Settings {
    pub fn new() -> Settings {
        Settings { aimbot: false }
    }
}
