use sdl2_sys::SDL_Event;

use crate::define_hook;

define_hook!(
    PollEventHook,
    "PollEvent",
    isize,
    1,
    event,
    &mut SDL_Event
);
