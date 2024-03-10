use std::{
    alloc::{alloc, Layout},
    mem::ManuallyDrop,
};

use libc::c_void;

use crate::{cfn, d, draw::Draw, o, DRAW};

pub type SwapWindowFn = cfn!((), *mut sdl2_sys::SDL_Window);

pub unsafe extern "C-unwind" fn swap_window_hook(window: *mut sdl2_sys::SDL_Window) -> () {
    if DRAW.is_none() {
        let draw_ptr = alloc(Layout::new::<Draw>()) as *mut _ as *mut ManuallyDrop<Draw>;
        let draw = ManuallyDrop::new(Draw::init(window).unwrap());
        *draw_ptr = draw;
        DRAW = Some(draw_ptr as *mut _ as *mut c_void);
    }

    d!().run(window);

    (o!().hooks.swap_window.org)(window);
}
