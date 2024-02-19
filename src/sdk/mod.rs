use std::fmt::Debug;

use crate::*;

mea!(base_client);
mea!(base_engine);
mea!(entity_list);
mea!(entity);
mea!(collideable);
mea!(user_cmd);
mea!(engine_vgui);
mea!(cvar);
mea!(convar);
mea!(view_setup);
mea!(mat_surface);
mea!(panel);
mea!(weapon);
mea!(model_info);
mea!(render_view);
mea!(engine_trace);
mea!(material_system);
mea!(model_render);
mea!(game_movement);
mea!(predictions);
mea!(client_mode);
mea!(networkable);

pub type CBaseHandle = c_uint;
pub type ConCommand = *const c_void;
pub type HFont = c_uint;

#[allow(non_snake_case, non_camel_case_types, dead_code)]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VMatrix([[c_float; 4]; 4]);

#[allow(non_snake_case, non_camel_case_types, dead_code)]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Angles(pub c_float, pub c_float, pub c_float);

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct WithVmt<T> {
    pub vmt: *mut T,
}

impl<T: Copy> WithVmt<T> {
    pub unsafe fn c(&mut self) -> T {
        *self.vmt
    }
}

pub trait HasVmt<T> {
    fn get_vmt(&self) -> *mut T;
    fn set_vmt(&mut self, vmt: *mut T);
    unsafe fn c(&mut self) -> T;
}
impl<T: Copy> HasVmt<T> for WithVmt<T> {
    fn get_vmt(&self) -> *mut T {
        self.vmt
    }
    fn set_vmt(&mut self, vmt: *mut T) {
        self.vmt = vmt
    }
    unsafe fn c(&mut self) -> T {
        *self.vmt
    }
}
