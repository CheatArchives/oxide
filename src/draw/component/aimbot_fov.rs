use std::f32::consts::PI;

use crate::{c, draw::{colors::YELLOW, event::Event, frame::Frame}, o, s, sdk::entity::Entity};

use super::Component;

#[derive(Debug)]
pub struct AimbotFov {}

impl AimbotFov {
    fn should_draw(&self) -> bool {
        if !*s!().aimbot.enabled.lock().unwrap() || !*s!().aimbot.draw_fov.lock().unwrap(){
            return false;
        }

        let Some(p_local) = Entity::local() else {
            return false;
        };

        if ! c!(p_local, is_alive) {
            return false;
        }
        true
    }
}

impl Component for AimbotFov {
    fn draw(&mut self, frame: &mut Frame, _: isize, _: isize) {
        if !self.should_draw() {
            return;
        }
        let size = frame.window_size();
        let aimbot_fov = *s!().aimbot.fov.lock().unwrap() as f32;
        let Some(fov) = o!().fov else {return}; 

        let screen_fov = size.0 as f32 / size.1 as f32 / (4f32 / 3f32);
        let real_fov = (screen_fov * (fov / 360f32 * PI).tan()).atan();
        let radius = (aimbot_fov * PI / 360f32).tan() / (real_fov).tan() * size.0 as f32;

        frame.circle(size.0 / 2, size.1 / 2, radius, YELLOW, 200);
    }

    fn handle_event(&mut self, _: &mut Event) {}
}

