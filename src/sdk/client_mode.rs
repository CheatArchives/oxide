use crate::*;

pub type ClientMode = WithVmt<VMTClientMode>;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VMTClientMode {
    _pad1: [cfn!(c_void,c_void); 17],
    pub override_view: cfn!(c_void, &'static mut ClientMode,&'static mut ViewSetup),
    _pad2: [cfn!(c_void,c_void); 4],
    pub create_move: cfn!(bool, &'static mut ClientMode, f32,&'static mut UserCmd),
}
