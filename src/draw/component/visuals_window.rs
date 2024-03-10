use std::sync::{Arc, Mutex};

use crate::{
    draw::{event::Event, frame::Frame},
    s,
};

use super::{
    base::{checkbox::Checkbox, float_input::FloatInput, window::Window},
    Component, Components,
};

#[derive(Debug)]
pub struct VisualsWindow {
    window: Window,
}

impl VisualsWindow {
    pub fn new(visible: Arc<Mutex<bool>>) -> VisualsWindow {
        let mut components = Components::new();

        components.add(Checkbox::new(
            "third person",
            s!().visual.third_person.clone(),
            10,
            10,
        ));

        components.add(FloatInput::new("fov", 10, 30, 100, s!().visual.fov.clone()));
        let window = Window::new("VISUALS".to_owned(), visible, components);
        VisualsWindow { window }
    }
}

impl Component for VisualsWindow {
    fn draw(&mut self, frame: &mut Frame, root_x: isize, root_y: isize) {
        self.window.draw(frame, root_x, root_y);
    }

    fn handle_event(&mut self, event: &mut Event) {
        self.window.handle_event(event);
    }
}
