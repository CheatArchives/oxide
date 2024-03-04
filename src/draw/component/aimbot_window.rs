use std::sync::{Arc, Mutex};

use crate::*;

const HEADER_HEIGHT: isize = 50;

#[derive(Debug)]
pub struct AimbotWindow {
    window: window::Window,

}

impl AimbotWindow {
    pub fn new(visible: Arc<Mutex<bool>>) -> AimbotWindow {
        let mut components = Components::new();

        components.add(Checkbox::new("enable",Arc::new(Mutex::new(false)),10,10));

        let window = window::Window::new(0, 0,"AIMBOT".to_owned(), visible, components);
        AimbotWindow { window }
    }
}

impl RawComponent for AimbotWindow {
    fn draw(&mut self, frame: &mut Frame, root_x: isize, root_y: isize) {
        self.window.draw(frame, root_x, root_y);
    }

    fn handle_event(&mut self, event: *mut sdl2_sys::SDL_Event) {
        self.window.handle_event(event);
    }
}

impl Component for AimbotWindow {}
