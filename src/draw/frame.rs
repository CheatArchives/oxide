use sdl2_sys::*;

use crate::*;

pub struct Frame {
    window: *mut SDL_Window,
    pub renderer: *mut SDL_Renderer
}
impl Frame {
    pub fn new(window: *mut SDL_Window, renderer: *mut SDL_Renderer) -> Frame {
        Frame{
            window,
            renderer
        }
    }
    pub fn get_window_size(&self) -> (i32, i32) {
        let mut w = 0i32;
        let mut h = 0i32;

        unsafe {
            SDL_GetWindowSize(self.window, &mut w, &mut h);
        }
        return (w, h);
    }
}
