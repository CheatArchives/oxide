use sdl2_sys::{SDL_GL_MakeCurrent, SDL_ShowCursor, SDL_ENABLE};

use crate::*;

pub type SwapWindowFn = cfn!(c_void, *mut sdl2_sys::SDL_Window);

pub unsafe extern "C-unwind" fn swap_window_hook(window: *mut sdl2_sys::SDL_Window) -> c_void {
    if DRAW.is_none() {
        let draw_ptr = alloc(Layout::new::<Draw>()) as *mut _ as *mut Draw;
        *draw_ptr = Draw::init(window).unwrap();
        DRAW = Some(draw_ptr as *mut _ as *mut c_void);
    }

    draw!().run(window);

    (oxide!().hooks.swap_window.org)(window)
}
