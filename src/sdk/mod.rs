use std::fmt::Debug;
use crate::{cfn,impl_has_vmt};

pub use derivative::Derivative;

pub mod base_client;
pub mod base_engine;
pub mod entity_list;
pub mod entity;
pub mod collideable;
pub mod user_cmd;
pub mod engine_vgui;
pub mod cvar;
pub mod convar;
pub mod view_setup;
pub mod mat_surface;
pub mod panel;
pub mod weapon;
pub mod model_info;
pub mod render_view;
pub mod engine_trace;
pub mod material_system;
pub mod model_render;
pub mod game_movement;
pub mod predictions;
pub mod client_mode;
pub mod networkable;
pub mod condition;
pub mod global_vars;
pub mod player_class;

pub type CBaseHandle = usize;
pub type ConCommand = *const u8;
pub type HFont = usize;

pub type VMatrix = [[f32; 4]; 4];

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
         self.vmt = vmt
    }
}
