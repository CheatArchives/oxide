use std::{
    intrinsics::{size_of_val, volatile_store},
    mem::MaybeUninit,
    ptr::write_volatile,
    usize,
};

use libc::{dladdr, dlsym, posix_fadvise, wait, Dl_info};

use crate::*;

static SWAPWINDOW_OFFSET: usize = 0xFD648;
static POLLEVENT_OFFSET: usize = 0xFCF64;

type SwapWindowFn = cfn!(c_void, &sdl2::video::Window);

mea!(interfaces);
#[derive(Debug, Clone, Copy)]
pub struct Oxide {
    pub interfaces: Interfaces,
    pub swap_window_ptr: *const SwapWindowFn,
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

unsafe extern "C-unwind" fn swap_window(window: &sdl2::video::Window) -> c_void {

    MaybeUninit::uninit().assume_init()
}
impl Oxide {
    pub unsafe fn init() -> Result<Oxide, Box<dyn Error>> {
        let sdl_handle =
            get_handle("./bin/libSDL2-2.0.so.0")? as *const _ as *const *const *const c_void;
        let swap_window_ptr = ((*sdl_handle) as usize + SWAPWINDOW_OFFSET) as *mut SwapWindowFn;

        debug!("b {:?}", swap_window_ptr);
        *swap_window_ptr = swap_window;
        debug!("a {:?}", swap_window_ptr);

        let oxide = Oxide {
            interfaces: Interfaces::create()?,
            swap_window_ptr,
        };

        (*oxide.interfaces.client_mode.get_vmt()).CreateMove = create_move_hook;

        Ok(oxide)
    }
    pub unsafe fn close(self) {
        self.interfaces.restore();
    }
}
