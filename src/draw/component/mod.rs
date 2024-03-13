use super::{event::Event, frame::Frame};

pub mod aimbot_fov;
pub mod base;
pub mod overlay;
pub mod visuals_window;
pub mod aimbot_window;
pub mod movement_window;

pub trait Component: std::fmt::Debug {
    fn draw(&mut self, frame: &mut Frame, root_x: isize, root_y: isize);
    fn handle_event(&mut self, event: &mut Event);
}

#[derive(Debug)]
pub struct Components(Vec<Box<dyn Component>>);

impl Components {
    pub fn new() -> Components {
        Components(Vec::new())
    }
    pub fn add(&mut self, component: impl Component + 'static) {
        self.0.push(Box::new(component));
    }
    pub fn draw(&mut self, frame: &mut Frame, root_x: isize, root_y: isize) {
        for component in &mut self.0 {
            component.draw(frame, root_x, root_y)
        }
    }
    pub fn handle_event(&mut self, event: &mut Event) {
        self.0.reverse();

        for component in &mut self.0 {
            if event.handled {
                break;
            }
            component.handle_event(event)
        }
        self.0.reverse();
    }
}
