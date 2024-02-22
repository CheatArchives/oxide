use crate::*;

pub type CVar = WithVmt<VMTCVar>;



#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VMTCVar{
    _pad: [u8;4*12],
    pub find_var: cfn!(&'static mut ConVar, &'static CVar, &CStr),
    pub find_var_const: cfn!(&'static ConVar, &'static CVar, &CStr),
    pub find_command: cfn!(&'static mut ConCommand, &'static CVar, &CStr),
    pub find_command_const: cfn!(&'static ConCommand, &'static CVar, &CStr),
}

