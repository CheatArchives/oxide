use std::fmt::Debug;

use crate::*;

module_export!(base_client);
module_export!(base_engine);
module_export!(entity_list);
module_export!(entity);
module_export!(collideable);
module_export!(user_cmd);
module_export!(engine_vgui);
module_export!(cvar);
module_export!(convar);
module_export!(view_setup);
module_export!(mat_surface);
module_export!(panel);
module_export!(weapon);
module_export!(model_info);
module_export!(render_view);
module_export!(engine_trace);
module_export!(material_system);
module_export!(model_render);
module_export!(game_movement);
module_export!(predictions);
module_export!(client_mode);
module_export!(networkable);
module_export!(condition);
module_export!(global_vars);
module_export!(player_class);

pub type CBaseHandle = usize;
pub type ConCommand = *const c_void;
pub type HFont = usize;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct VMatrix([[f32; 4]; 4]);

#[repr(C)]
#[derive(Debug, Clone)]
pub struct WithVmt<T: 'static> {
    pub vmt: *mut T,
}

pub trait HasVmt<T: 'static> {
    type VMTType = T;
    fn get_vmt(&self) -> &'static T;
    fn set_vmt(&mut self, vmt: *mut T);
}
impl<T: 'static + Clone + Debug> HasVmt<T> for WithVmt<T> {
    fn get_vmt(&self) -> &'static T {
        unsafe { &*self.vmt }
    }
    fn set_vmt(&mut self, vmt: *mut T) {
        unsafe{
            self.vmt = vmt
        }
    }
}
