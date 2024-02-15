use crate::*;

pub type ClientMode = WithVmt<VMTClientMode>;

#[allow(non_snake_case, non_camel_case_types, dead_code)]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct VMTClientMode {
    _pad1: [u8; 4 * 17],
    pub OverrideView: cfn!(c_void, *mut ClientMode,*mut ViewSetup),
    _pad2: [u8; 4 * 4],
    pub CreateMove: cfn!(bool, *mut ClientMode, c_float,*mut UserCmd),
}
