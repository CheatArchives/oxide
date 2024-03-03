use std::{backtrace::Backtrace, collections::HashMap, mem::MaybeUninit, ops::Deref, usize};

use libc::{dlclose, dlopen, wait};
use sdl2_sys::{SDL_Event, SDL_EventType, SDL_Scancode};

use crate::*;

module_export!(interfaces);
module_export!(hooks);
module_export!(cheats);

#[derive(Debug, Clone)]
pub struct Oxide {
    pub interfaces: Interfaces,
    pub hooks: Hooks,
    pub global_vars: &'static GlobalVars,
    pub cheats: Cheats,
    pub fov: f32
}

impl Oxide {
    pub unsafe fn init() -> Result<Oxide, std::boxed::Box<dyn Error>> {
        let interfaces = Interfaces::init()?;
        let hooks = Hooks::init(&interfaces);
        let cheats = Cheats::init();

        let global_vars = Oxide::get_global_vars(interfaces.base_client.interface_ref());

        #[allow(invalid_value)]
        let oxide = Oxide {
            interfaces,
            cheats,
            hooks,
            global_vars,
            fov: 0f32,
        };

        Ok(oxide)
    }
    unsafe fn get_global_vars(base_client: &BaseClient) -> &'static mut GlobalVars {
        let hud_update_addr = (*base_client.vmt).hud_update as usize;
        let global_vars: &'static mut &'static mut &'static mut GlobalVars =
            transmute(hud_update_addr + 9);
        **global_vars
    }
    pub unsafe fn handle_event(&mut self, event: *mut SDL_Event) {
        match transmute::<u32, SDL_EventType>((*event).type_) {
            SDL_EventType::SDL_KEYDOWN => {
                let key = (*event).key.keysym.scancode;
                match key {
                    SDL_Scancode::SDL_SCANCODE_LSHIFT => {
                        self.cheats.aimbot.shoot_key_pressed = true
                    }
                    _ => (),
                }
            }
            SDL_EventType::SDL_KEYUP => {
                let key = (*event).key.keysym.scancode;
                match key {
                    SDL_Scancode::SDL_SCANCODE_LSHIFT => {
                        self.cheats.aimbot.shoot_key_pressed = false
                    }
                    _ => (),
                }
            }
            _ => (),
        };
    }
    pub fn self_unload() {
        let lib_path = CString::new("/tmp/liboxide.so").unwrap();
        unsafe {
            let handle = dlopen(lib_path.as_ptr(), 6);
            dlclose(handle);
            dlclose(handle);
        }
    }
    pub fn restore(&mut self) {
        self.interfaces.restore();
        self.hooks.restore();
    }
}
