use std::f32::consts::PI;

use crate::*;

#[derive(Debug)]
pub struct AimbotFov {}

impl AimbotFov {
    fn should_draw(&self) -> bool {
        if !settings!().aimbot || !settings!().aimbot_draw_fov{
            return false;
        }

        let Some(p_local) = Entity::local() else {
            return false;
        };

        if !unsafe { call!(p_local, is_alive) } {
            return false;
        }
        true
    }
}

impl RawComponent for AimbotFov {
    fn draw(&mut self, frame: &mut Frame, root_x: isize, root_y: isize) {
        if !self.should_draw() {
            return;
        }
        let size = frame.window_size();
        let aimbot_fov = settings!().aimbot_fov as f32;
        let fov = oxide!().fov;

        let screen_fov = size.0 as f32 / size.1 as f32 / (4f32 / 3f32);
        let real_fov = (screen_fov * (fov / 360f32 * PI).tan()).atan();
        let radius = (aimbot_fov * PI / 360f32).tan() / (real_fov).tan() * size.0 as f32;

        frame.circle(size.0 / 2, size.1 / 2, radius, YELLOW, 200);
    }

    fn handle_event(&mut self, event: &mut Event) {}
}

impl Component for AimbotFov {}
