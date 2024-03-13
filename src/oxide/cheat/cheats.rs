use std::{collections::HashMap, mem::{transmute, ManuallyDrop}};

use crate::{draw::event::Event, oxide::hook::hooks::Hooks};

use super::{aimbot::Aimbot, Cheat};

#[derive(Debug)]
pub struct Cheats (
    pub  HashMap<String,Box<dyn Cheat>>,
);

impl Cheats {
    pub fn init(hooks: &mut Hooks) -> Cheats {
        let cheats = HashMap::new();
        let mut cheats = Cheats ( cheats );

        let mut aimbot = Aimbot::init();
        aimbot.hook(hooks);
        cheats.add(aimbot,Aimbot::name());

        cheats
    }
    pub fn handle_event(&mut self,event: &mut Event) {
        for (_,cheat) in &mut self.0 {
            cheat.handle_event(event)
        }
    }
    fn add(&mut self, cheat: impl Cheat + 'static,name: &str) {
        self.0.insert(name.to_owned(),Box::new(cheat));
    }
    pub fn get<T>(&mut self, name: &str) -> ManuallyDrop<&mut Box<T>>{
        unsafe { ManuallyDrop::new(transmute(self.0.get_mut(name).unwrap())) }
    }
}
