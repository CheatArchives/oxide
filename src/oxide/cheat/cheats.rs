use super::aimbot::Aimbot;


#[derive(Debug, Clone)]
pub struct Cheats {
    pub aimbot: Aimbot,
}

impl Cheats {
    pub fn init() -> Cheats {
        let aimbot = Aimbot::init();
        Cheats { aimbot }
    }
}
