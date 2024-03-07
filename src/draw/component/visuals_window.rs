use std::sync::{Arc, Mutex};

use sdl2_sys::SDL_Scancode;

use crate::*;

const HEADER_HEIGHT: isize = 50;

#[derive(Debug)]
pub struct VisualsWindow {
    window: window::Window,
}

impl VisualsWindow {
    pub fn new(visible: Arc<Mutex<bool>>) -> VisualsWindow {
        let mut components = Components::new();

        components.add(Checkbox::new(
            "third person",
            settings!().visual.third_person.clone(),
            10,
            10,
        ));

        components.add(FloatInput::new(
            "fov",
            30,
            10,
            100,
            settings!().visual.fov.clone()
        ));
        let window = window::Window::new("VISUALS".to_owned(), visible, components);
        VisualsWindow { window }
    }
}

impl RawComponent for VisualsWindow {
    fn draw(&mut self, frame: &mut Frame, root_x: isize, root_y: isize) {
        self.window.draw(frame, root_x, root_y);
    }

    fn handle_event(&mut self, event: &mut Event) {
        self.window.handle_event(event);
    }
}

impl Component for VisualsWindow {}
