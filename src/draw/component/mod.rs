
use crate::*;
use freetype_sys::*;
use libc::CS;
use sdl2_sys::*;

module_export!(checkbox);
module_export!(aimbot_fov);
module_export!(overlay);

pub trait Component {
    fn draw(&mut self,frame: &mut Frame, root_x: isize, root_y: isize);
    fn handle_event(&mut self, event: *mut SDL_Event);
}
pub trait ComponentDebug: component::Component + std::fmt::Debug {}
