use std::{
    intrinsics::{size_of_val, volatile_store}, mem::MaybeUninit, os::unix::fs::MetadataExt, ptr::{self, write_volatile}, usize
};

use libc::{dladdr, dlsym, posix_fadvise, wait, Dl_info};
use sdl2_sys::*;

use crate::*;

static SWAPWINDOW_OFFSET: usize = 0xFD648;
static POLLEVENT_OFFSET: usize = 0xFCF64;

type SwapWindowFn = cfn!(c_void, *mut sdl2_sys::SDL_Window);

mea!(interfaces);
#[derive(Debug, Clone, Copy)]
pub struct Oxide {
    pub interfaces: Interfaces,
    pub swap_window_ptr: *mut SwapWindowFn,
    pub swap_window_old: SwapWindowFn,
    pub ctx: Option<*mut c_void>,
    pub ctx_old: Option<*mut c_void>
}

unsafe extern "C-unwind" fn create_move_hook(
    client_mode: *mut ClientMode,
    input_sample_time: c_float,
    cmd: *mut UserCmd,
) -> bool {
    //debug!("cmd2: {:?}", *cmd);
    (*cmd).viewangles.0 = 0.0;
    (*cmd).viewangles.1 = 0.0;
    (*cmd).viewangles.2 = 0.0;
    true
}


unsafe extern "C-unwind" fn swap_window(window: *mut sdl2_sys::SDL_Window) -> c_void {

    if o!().ctx.is_none() {
        o!().ctx_old = Some(SDL_GL_GetCurrentContext());
        
        o!().ctx = Some(SDL_GL_CreateContext(window));
    }


    SDL_GL_MakeCurrent(window,o!().ctx.unwrap());
    let surface = SDL_GetWindowSurface(window);
    SDL_FillRect(surface, std::ptr::null(), 0x000000);
    let rect = SDL_Rect {x:100,y:100,w:100,h:100};
    SDL_FillRect(
        surface,
        &rect,
        SDL_MapRGB((*surface).format, 0xFF, 0x00, 0x00),
    );
    SDL_UpdateWindowSurface(window);


    SDL_GL_MakeCurrent(window,o!().ctx_old.unwrap());
    (o!().swap_window_old)(window);
    MaybeUninit::uninit().assume_init()
}

impl Oxide {
    pub unsafe fn init() -> Result<Oxide, Box<dyn Error>> {
        let sdl_handle =
            get_handle("./bin/libSDL2-2.0.so.0")? as *const _ as *const *const *const c_void;

        let swap_window_ptr = ((*sdl_handle) as usize + SWAPWINDOW_OFFSET) as *mut SwapWindowFn;
        let swap_window_old = *swap_window_ptr;

        *swap_window_ptr = swap_window;

        let oxide = Oxide {
            interfaces: Interfaces::create()?,
            swap_window_ptr,
            swap_window_old,
            ctx: Option::None,
            ctx_old: Option::None,
        };

        (*oxide.interfaces.client_mode.get_vmt()).CreateMove = create_move_hook;

        Ok(oxide)
    }
    pub unsafe fn unload(self) {
        self.interfaces.restore();
        info!("restoring sdl");
        debug!(
            "form {:?} to {:?}",
            *self.swap_window_ptr, self.swap_window_old
        );
        *self.swap_window_ptr = self.swap_window_old;
    }
}
