use std::{error::Error, ffi::CString, mem::transmute};

use libc::{dlclose, dlopen};
use sdl2_sys::SDL_Event;

use crate::{
    d,
    draw::event::{Event, EventType},
    math::{angles::Angles, vector::Vector3},
    oxide::{cheat::cheats::Cheats, hooks::Hooks, interfaces::Interfaces},
    s,
    sdk::{base_client::BaseClient, entity::Entity, global_vars::GlobalVars},
    util::sigscanner::find_sig,
    DRAW,
};

pub mod cheat;
pub mod hooks;
pub mod interfaces;
pub mod paint;

#[derive(Debug, Clone)]
pub struct Oxide {
    pub interfaces: Interfaces,
    pub hooks: Hooks,
    pub global_vars: &'static GlobalVars,
    pub cheats: Cheats,
    pub fov: f32,
    pub get_bone_position_fn: GetBonePositionFn,
}
pub type GetBonePositionFn =
    unsafe extern "C-unwind" fn(&Entity, usize, &mut Vector3, &mut Angles) -> ();

impl Oxide {
    pub unsafe fn init() -> Result<Oxide, std::boxed::Box<dyn Error>> {
        let sig =
            "55 89 E5 53 8D 5D ? 83 EC 44 8B 45 ? 89 5C 24 ? 89 44 24 ? 8B 45 ? 89 04 24 E8 ? ? ? ? 8B 45";
        let get_bone_position_fn = transmute(find_sig("./tf/bin/client.so", &sig));
        let interfaces = Interfaces::init()?;
        let hooks = Hooks::init(&interfaces);
        let cheats = Cheats::init();

        let global_vars = Oxide::get_global_vars(interfaces.base_client.interface_ref());

        let oxide = Oxide {
            interfaces,
            cheats,
            hooks,
            global_vars,
            fov: 0f32,
            get_bone_position_fn,
        };

        Ok(oxide)
    }
    unsafe fn get_global_vars(base_client: &BaseClient) -> &'static mut GlobalVars {
        let hud_update_addr = (*base_client.vmt).hud_update as usize;
        let global_vars: &'static mut &'static mut &'static mut GlobalVars =
            transmute(hud_update_addr + 9);
        **global_vars
    }
    pub unsafe fn handle_event(&mut self, event: *mut SDL_Event) -> bool {
        let mut event = Event::from(unsafe { *event });
        let aimbot_key = *s!().aimbot.key.lock().unwrap();

        match event.r#type {
            EventType::KeyDown(key) => {
                if key == aimbot_key {
                    self.cheats.aimbot.shoot_key_pressed = true
                }
            }
            EventType::KeyUp(key) => {
                if key == aimbot_key {
                    self.cheats.aimbot.shoot_key_pressed = false
                }
            }
            _ => (),
        }

        if DRAW.is_some() {
            d!().handle_event(&mut event);
        }
        return event.handled;
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

    pub fn global_vars(&self) -> &GlobalVars {
        self.global_vars
    }

    pub fn global_vars_mut(&mut self) -> &mut &'static GlobalVars {
        &mut self.global_vars
    }
}
