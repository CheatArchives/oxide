use std::mem::MaybeUninit;

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

unsafe extern "C-unwind" fn level_init_post_entity(base_client: *mut BaseClient) -> c_void {
    debug!("INIT!");
    MaybeUninit::<c_void>::uninit().assume_init()
}
impl Oxide {
    pub unsafe fn init() -> Result<Oxide, Box<dyn Error>>{
        let oxide = Oxide {
            interfaces: Interfaces::create()?,
        };
        let interface = oxide.interfaces.client_mode.interface_ref.read();
        let vmt: *mut VMTClientMode = interface.vmt;
        debug!("my {:?}",create_move_hook as *mut c_void);
        debug!("org {:?}",vmt.read().CreateMove as *mut c_void);
        vw!(vmt.read().CreateMove, create_move_hook);
        debug!("altered {:?}",vmt.read().CreateMove as *mut c_void);

        let interface = oxide.interfaces.base_client.interface_ref.read();
        let vmt: *mut VMTBaseClient = interface.vmt;
        debug!("my {:?}",level_init_post_entity as *mut c_void);
        debug!("org {:?}",vmt.read().LevelInitPostEntity as *mut c_void);
        vw!(vmt.read().LevelInitPostEntity, level_init_post_entity);
        debug!("altered {:?}",vmt.read().LevelInitPostEntity as *mut c_void);

        Ok(oxide)
    }
    pub unsafe fn close(self) {
        self.interfaces.restore()
    }
}

unsafe impl Send for Oxide {}
