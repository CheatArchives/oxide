use sdl2_sys::*;

use crate::*;

pub struct Frame {
    window: *mut SDL_Window,
    pub renderer: *mut SDL_Renderer,
    pub window_size: (isize, isize)
}
impl Frame {
    pub fn new(window: *mut SDL_Window, renderer: *mut SDL_Renderer) -> Frame {
        Frame{
            window,
            renderer,
            window_size: Frame::window_size(window)
        }
    }
    pub fn window_size(window: *mut SDL_Window) -> (isize, isize) {
        let mut w = 0i32;
        let mut h = 0i32;

        unsafe {
            SDL_GetWindowSize(window, &mut w, &mut h);
        }
        return (w as isize, h as isize);
    }
}
