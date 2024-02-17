use crate::*;

#[allow(non_snake_case,non_camel_case_types,dead_code)]
#[repr(C)]
#[derive(Debug,Clone, Copy)]
pub struct UserCmd{
    pub vmt: *const c_void,
    pub command_number: c_int,
    pub tick_count:c_int,
    pub viewangles: Angles,
    pub forwardmove: c_float,
    pub sidemove: c_float,
    pub upmove: c_float,
    pub buttons: c_int,
    pub impulse: u8,
    pub weaponselect: isize,
    pub weaponsubtype: isize,
    pub random_seed: isize,
    pub mousedx: i16,
    pub mousedy: i16,
    pub hasbeenpredicted: bool,
}
