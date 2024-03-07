use std::sync::{Arc, Mutex};

use sdl2_sys::SDL_Scancode;

use crate::*;

const HEADER_HEIGHT: isize = 50;

#[derive(Debug)]
pub struct AimbotWindow {
    window: window::Window,
}

impl AimbotWindow {
    pub fn new(visible: Arc<Mutex<bool>>) -> AimbotWindow {
        let mut components = Components::new();

        components.add(Checkbox::new(
            "enable",
            settings!().aimbot.enabled.clone(),
            10,
            10,
        ));
        components.add(Checkbox::new(
            "draw_fov",
            settings!().aimbot.draw_fov.clone(),
            10,
            30,
        ));
        components.add(FloatInput::new("aimbot fov",10, 50, 100, settings!().aimbot.fov.clone()));
        components.add(KeyInput::new(10, 75, 100, settings!().aimbot.key.clone()));

        let window = window::Window::new("AIMBOT".to_owned(), visible, components);
        AimbotWindow { window }
    }
}

impl RawComponent for AimbotWindow {
    fn draw(&mut self, frame: &mut Frame, root_x: isize, root_y: isize) {
        self.window.draw(frame, root_x, root_y);
    }

    fn handle_event(&mut self, event: &mut Event) {
        self.window.handle_event(event);
    }
}

impl Component for AimbotWindow {}
