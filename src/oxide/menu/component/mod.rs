
use crate::*;
use freetype_sys::*;
use libc::CS;
use sdl2_sys::*;

module_export!(checkbox);

pub trait Component {
    fn draw(&mut self,draw: &mut Draw, root_x: isize, root_y: isize);
    fn handle_event(&mut self, event: *mut SDL_Event);
}
