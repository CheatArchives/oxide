use crate::*;
use freetype_sys::*;
use libc::CS;
use sdl2_sys::*;

module_export!(checkbox);
module_export!(aimbot_fov);
module_export!(overlay);
module_export!(button);
module_export!(window);

pub trait RawComponent {
    fn draw(&mut self, frame: &mut Frame, root_x: isize, root_y: isize);
    fn handle_event(&mut self, event: *mut SDL_Event);
}

pub trait Component: component::RawComponent + std::fmt::Debug {}

#[derive(Debug)]
pub struct Components(Vec<Box<dyn Component>>);

impl Components {
    pub fn new() -> Components {
        Components(Vec::new())
    }
    pub fn add(&mut self,component: impl Component + 'static) {
        self.0.push(Box::new(component));
    }
    pub fn draw(&mut self, frame: &mut Frame) {
        for component in &mut self.0 {
            component.draw(frame, 0, 0)
        }
    }
    pub fn handle_event(&mut self, event: *mut SDL_Event) {
        for component in &mut self.0 {
            component.handle_event(event)
        }
    }
}
