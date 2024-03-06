
use crate::*;
use freetype_sys::*;
use libc::CS;
use sdl2_sys::*;

module_export!(aimbot_fov);
module_export!(overlay);
module_export!(base);
module_export!(aimbot_window);
module_export!(visuals_window);

pub trait RawComponent {
    fn draw(&mut self, frame: &mut Frame, root_x: isize, root_y: isize);
    fn handle_event(&mut self, event: &mut Event);
}

pub trait Component: component::RawComponent + std::fmt::Debug {}

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
