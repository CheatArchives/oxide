use crate::*;

pub type CVar = WithVmt<VMTCVar>;



#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VMTCVar{
    _pad: [u8;4*12],
    pub find_var: cfn!(*mut ConVar, *const CVar, *const c_char),
    pub find_var_const: cfn!(*const ConVar, *const CVar, *const c_char),
    pub find_command: cfn!(*mut ConCommand, *const CVar, *const c_char),
    pub find_command_const: cfn!(*const ConCommand, *const CVar, *const c_char),
}

