use std::{
    intrinsics::{size_of_val, volatile_store},
    mem::MaybeUninit,
    ptr::write_volatile,
    usize,
};

use libc::{dladdr, dlsym, posix_fadvise, wait, Dl_info};

use crate::*;

mea!(interfaces);
#[derive(Debug, Clone, Copy)]
pub struct Oxide {
    pub interfaces: Interfaces,
}

unsafe extern "C-unwind" fn create_move_hook(
    client_mode: *mut ClientMode,
    input_sample_time: c_float,
    cmd: *mut UserCmd,
) -> bool {
    debug!("cmd1: {:?}", *cmd);
    true
}

unsafe extern "C-unwind" fn level_init_post_entity(base_client: *mut BaseClient) -> c_void {
    debug!("INIT!");
    MaybeUninit::<c_void>::uninit().assume_init()
}
impl Oxide {
    pub unsafe fn init() -> Result<Oxide, Box<dyn Error>> {
        let oxide = Oxide {
            interfaces: Interfaces::create()?,
        };

        (*oxide.interfaces.client_mode.get_vmt()).CreateMove = create_move_hook;

        Ok(oxide)
    }
    pub unsafe fn close(self) {
        self.interfaces.restore()
    }
}

