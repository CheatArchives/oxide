use libc::wait;

use crate::*;

mea!(interfaces);
#[derive(Debug)]
pub struct Oxide {
    interfaces: Interfaces,
}

unsafe extern "C-unwind" fn create_move_hook(client_mode: *mut ClientMode, input_sample_time: c_float, cmd: *mut UserCmd) -> bool {
    debug!("CREATE MOVE!");
    return true
}

impl Oxide {
    pub unsafe fn init() -> Result<Oxide, Box<dyn Error>>{
        let oxide = Oxide {
            interfaces: Interfaces::create()?,
        };
        let mut interface = oxide.interfaces.client_mode.interface_ref.read();
        interface.vmt = 0 as *mut _;
        oxide.interfaces.client_mode.interface_ref.read().vmt.read().CreateMove = create_move_hook;


        Ok(oxide)
    }
    pub unsafe fn close(self) {
        self.interfaces.restore()
    }
}

unsafe impl Send for Oxide {}
