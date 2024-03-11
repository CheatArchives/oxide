//pub unsafe extern "C-unwind" fn poll_event_hook(event: *mut SDL_Event) -> isize {
//    let handled = o!().handle_event(transmute(event));
//
//    match handled {
//        true => (*event).type_ = 0,
//        false => (),
//    }
//    //(o!().hooks.poll_event.org)(event)
//    1
//}

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
