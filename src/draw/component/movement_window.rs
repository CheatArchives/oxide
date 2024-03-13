use std::sync::{Arc, Mutex};

use crate::{draw::{event::Event, frame::Frame}, s};

use super::{base::{checkbox::Checkbox, window::Window}, Component, Components};


#[derive(Debug)]
pub struct MovementWindow {
    window: Window,
}

impl MovementWindow {
    pub fn new(visible: Arc<Mutex<bool>>) -> MovementWindow {
        let mut components = Components::new();

        components.add(Checkbox::new("bhop", s!().movement.bhop.clone(), 10, 10));

        let window = Window::new("Movement".to_owned(), visible, components);
        MovementWindow { window }
    }
}

impl Component for MovementWindow {
    fn draw(&mut self, frame: &mut Frame, root_x: isize, root_y: isize) {
        self.window.draw(frame, root_x, root_y);
    }

    fn handle_event(&mut self, event: &mut Event) {
        self.window.handle_event(event);
    }
}
