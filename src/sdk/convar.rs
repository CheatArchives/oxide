use crate::*;


#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VMTConVar {
    _pad: [u8; 4*14],
    pub internal_set_value: cfn!(c_void, *const ConVar , *const c_char),
    pub internal_set_float_value: cfn!(c_void, *const ConVar,c_float , bool),
    pub internal_set_int_value: cfn!(c_void, *const ConVar, c_int),
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ConVar {
    vmt: &'static VMTConVar,
    _pad: [u8; 0x18],
    parent: *const ConVar,
    default_value: *const c_char,
    string: *const c_char,
    string_length: c_int,
    float_value: c_float,
    int_value: c_int,
    has_min: bool,
    min_val: c_float,
    has_max: bool,
    max_val: c_float,
    has_comp_min: bool,
    comp_min_val: c_float,
    has_comp_max: bool,
    comp_max_val: c_float,
    competitive_restrictions: bool,
    change_callback: *const c_void,
}
