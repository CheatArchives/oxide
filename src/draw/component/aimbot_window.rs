use std::sync::{Arc, Mutex};


use crate::{draw::{event::Event, frame::Frame}, s};

use super::{base::{checkbox::Checkbox, float_input::FloatInput, key_input::KeyInput, window::Window}, Component, Components, RawComponent};

#[derive(Debug)]
pub struct AimbotWindow {
    window: Window,
}

impl AimbotWindow {
    pub fn new(visible: Arc<Mutex<bool>>) -> AimbotWindow {
        let mut components = Components::new();

        components.add(Checkbox::new(
            "enable",
            s!().aimbot.enabled.clone(),
            10,
            10,
        ));
        components.add(Checkbox::new(
            "draw_fov",
            s!().aimbot.draw_fov.clone(),
            10,
            30,
        ));
        components.add(FloatInput::new(
            "aimbot fov",
            10,
            50,
            100,
            s!().aimbot.fov.clone(),
        ));
        components.add(KeyInput::new(10, 75, 100, s!().aimbot.key.clone()));

        let window = Window::new("AIMBOT".to_owned(), visible, components);
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
