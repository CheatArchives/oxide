use crate::define_hook;

define_hook!(
    SwapWindowHook,
    "SwapWindow",
    (),
    (),
    window,
    *mut sdl2_sys::SDL_Window
);
