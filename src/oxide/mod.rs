use std::{ops::Deref, usize};

use crate::*;

module_export!(interfaces);
module_export!(menu);
module_export!(hooks);
module_export!(cheats);

#[derive(Debug, Clone, Copy)]
pub struct Oxide {
    pub interfaces: Interfaces,
    pub hooks: Hooks,
    pub global_vars: &'static GlobalVars,
    pub cheats: Cheats,
}

impl Oxide {
    pub unsafe fn init() -> Result<Oxide, std::boxed::Box<dyn Error>> {
        let interfaces = Interfaces::init()?;
        let hooks = Hooks::init(&interfaces)?;
        let cheats = Cheats::init();

        let global_vars = Oxide::get_global_vars(*interfaces.base_client.interface_ref());

        let oxide = Oxide {
            interfaces,
            hooks,
            global_vars,
            cheats,
        };



        Ok(oxide)
    }
    unsafe fn get_global_vars(base_client: BaseClient) -> &'static mut GlobalVars {
        let hud_update_addr = (*base_client.vmt).hud_update as usize;
        let global_vars:&'static mut &'static mut &'static mut GlobalVars = transmute(hud_update_addr + 9);
        **global_vars
    }
    pub unsafe fn unload(&mut self) {
        self.interfaces.restore();
        self.hooks.restore();
    }
}
