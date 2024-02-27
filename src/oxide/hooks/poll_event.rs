use sdl2_sys::SDL_Event;

use crate::*;
pub type PollEventFn = cfn!(isize, *mut SDL_Event);


pub unsafe extern "C-unwind" fn poll_event_hook(event: *mut SDL_Event) -> isize {
    menu!().handle_event(transmute(event));
    (oxide!().hooks.poll_event.org)(event)
}

