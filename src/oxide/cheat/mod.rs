use crate::draw::event::Event;

use super::hook::hooks::Hooks;

pub mod cheats;

pub mod aimbot;

pub trait Cheat: std::fmt::Debug {
    fn hook(&mut self, hooks: &mut Hooks);
    fn handle_event(&mut self, event: &mut Event);
}
