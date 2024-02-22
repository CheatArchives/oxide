use crate::*;


#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VMTConVar {
    _pad: [u8; 4*14],
    pub internal_set_value: cfn!(c_void, &'static ConVar , &CStr),
    pub internal_set_float_value: cfn!(c_void, &'static ConVar,f32 , bool),
    pub internal_set_int_value: cfn!(c_void, &'static ConVar, isize),
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ConVar {
    vmt: &'static VMTConVar,
    _pad: [u8; 0x18],
    parent: &'static ConVar,
    default_value: &'static CStr,
    string: &'static CStr,
    string_length: isize,
    float_value: f32,
    int_value: isize,
    has_min: bool,
    min_val: f32,
    has_max: bool,
    max_val: f32,
    has_comp_min: bool,
    comp_min_val: f32,
    has_comp_max: bool,
    comp_max_val: f32,
    competitive_restrictions: bool,
    change_callback: &'static c_void,
}
