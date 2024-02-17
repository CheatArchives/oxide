use crate::*;

pub type CVar = WithVmt<VMTCVar>;



#[allow(non_snake_case, non_camel_case_types, dead_code)]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VMTCVar{
    _pad: [u8;4*12],
    pub FindVar: cfn!(*mut ConVar, *const CVar, *const c_char),
    pub FundVar_const: cfn!(*const ConVar, *const CVar, *const c_char),
    pub FindCommand: cfn!(*mut ConCommand, *const CVar, *const c_char),
    pub FindCommand_const: cfn!(*const ConCommand, *const CVar, *const c_char),
}

