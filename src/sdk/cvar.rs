use std::ffi::{CStr, CString};

use crate::{c, i};

use self::convar::ConVar;

use super::*;

pub type CVar = WithVmt<VMTCVar>;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct VMTCVar {
    _pad: [u8; 4 * 12],
    pub find_var: cfn!(&'static mut ConVar, &CVar, *const i8),
    pub find_var_const: cfn!(&'static ConVar, &CVar, &CStr),
    pub find_command: cfn!(&mut ConCommand, &'static CVar, CStr),
    pub find_command_const: cfn!(&ConCommand, &'static CVar, CStr),
}

pub fn get_cvar(name: &str) -> &mut ConVar {
    let name = CString::new(name).unwrap();
    c!(i!(cvar), find_var, name.as_ptr())
}
