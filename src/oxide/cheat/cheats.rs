use crate::oxide::hooks::Hooks;

use super::{aimbot::Aimbot, Cheat};

#[derive(Debug)]
pub struct Cheats {
    pub cheats: Vec<Box<dyn Cheat>>,
}

impl Cheats {
    pub fn init(_: &mut Hooks) -> Cheats {
        let cheats = Vec::new();
        let mut cheats = Cheats { cheats };

        let aimbot = Aimbot::init();
        cheats.add(aimbot);

        cheats
    }
    fn add(&mut self, cheat: impl Cheat + 'static) {
        self.cheats.push(Box::new(cheat))
    }
}
