

use crate::*;

use self::{hooks::Hooks};

mea!(interfaces);
mea!(menu);
mea!(hooks);

#[derive(Debug, Clone, Copy)]
pub struct Oxide {
    pub interfaces: Interfaces,
    pub hooks: Hooks,
    pub global_vars: &'static GlobalVars,
}


impl Oxide {
    pub unsafe fn init() -> Result<Oxide, Box<dyn Error>> {
        let interfaces = Interfaces::init()?;
        let hooks = Hooks::init(&interfaces)?;

        let global_vars = &**(((*(*interfaces.base_client.interface_ref).vmt).HudUpdate as usize + 9)
            as *mut *mut *mut GlobalVars)
            .read_unaligned();
        let oxide = Oxide {
            interfaces,
            hooks,
            global_vars
        };


        Ok(oxide)
    }
    pub unsafe fn unload(self) {
        self.interfaces.restore();
        self.hooks.restore();
    }
}
