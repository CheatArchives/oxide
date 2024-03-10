use std::mem::transmute;

use sdl2_sys::SDL_Event;

use crate::{cfn, o};

pub type PollEventFn = cfn!(isize, *mut SDL_Event);

pub unsafe extern "C-unwind" fn poll_event_hook(event: *mut SDL_Event) -> isize {
    let handled = o!().handle_event(transmute(event));

    match handled {
        true => (*event).type_ = 0,
        false => (),
    }
    (o!().hooks.poll_event.org)(event)
}
