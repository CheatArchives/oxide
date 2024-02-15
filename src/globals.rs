use crate::*;
use std::error::Error;

use libc::{dlclose, dlerror, dlopen, dlsym, RTLD_LAZY, RTLD_NOLOAD};
use sdl2::sys::{SDL_Event, SDL_Window};


const SWAPWINDOW_OFFSET: usize = 0xFD648;
const POLLEVENT_OFFSET: usize = 0xFCF64;


pub unsafe fn init_globals() -> Result<(), Box<dyn Error>> {
    let sdl2_handle = get_handle::<c_void>("./bin/libSDL2-2.0.so.0")?;

    let swap_window_ptr: cfn!((), *const SDL_Window) =
        transmute(sdl2_handle.read() as usize + SWAPWINDOW_OFFSET);
    let pool_event_ptr: cfn!((), *const SDL_Event) =
        transmute(sdl2_handle.read() as usize + POLLEVENT_OFFSET);


    return Ok(());
}
