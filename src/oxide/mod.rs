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
    pub cheats: Cheats
}

impl Oxide {
    pub unsafe fn init() -> Result<Oxide, std::boxed::Box<dyn Error>> {
        let interfaces = Interfaces::init()?;
        let hooks = Hooks::init(&interfaces)?;
        let cheats = Cheats::init();

        let global_vars = &**(((*(*interfaces.base_client.interface_ref).vmt).HudUpdate as usize
            + 9) as *mut *mut *mut GlobalVars)
            .read_unaligned();
        let oxide = Oxide {
            interfaces,
            hooks,
            global_vars,
            cheats
        };

        Ok(oxide)
    }
    pub unsafe fn unload(self) {
        self.interfaces.restore();
        self.hooks.restore();
    }
}
