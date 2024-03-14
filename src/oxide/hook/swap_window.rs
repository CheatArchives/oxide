use std::{
    alloc::{alloc, Layout},
    mem::ManuallyDrop,
};

use libc::c_void;

use crate::{d, define_hook, draw::Draw, DRAW};

fn subhooks(hook: &mut SwapWindowHook) {
    hook.before = Some(|window| unsafe {
        if DRAW.is_none() {
            let draw_ptr = alloc(Layout::new::<Draw>()) as *mut _ as *mut ManuallyDrop<Draw>;
            let draw = ManuallyDrop::new(Draw::init(window)?);
            *draw_ptr = draw;
            DRAW = Some(draw_ptr as *mut _ as *mut c_void);
        }
        d!().run(window);
        Ok(true)
    });
    hook.after = Some(|_, _| Ok(()));
}

define_hook!(
    SwapWindowHook,
    "SwapWindow",
    (),
    (),
    subhooks,
    window,
    *mut sdl2_sys::SDL_Window
);
