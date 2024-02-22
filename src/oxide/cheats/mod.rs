use crate::*;

module_export!(aimbot);

#[derive(Debug, Clone, Copy)]
pub struct Cheats {
    pub aimbot: Aimbot,
}

impl Cheats {
    pub fn init() -> Cheats {
        let aimbot = Aimbot::init();
        Cheats { aimbot }
    }
}
