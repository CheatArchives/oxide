use crate::*;

#[allow(non_snake_case,non_camel_case_types,dead_code)]
#[repr(C)]
#[derive(Debug,Clone, Copy)]
pub struct UserCmd{
    vmt: *const c_void,
    command_number: c_int,
    tick_count:c_int,
    viewangles: Angles,
    forwardmove: c_float,
    sidemove: c_float,
    upmove: c_float,
    buttons: c_int,
    impulse: u8,
    weaponselect: isize,
    weaponsubtype: isize,
    random_seed: isize,
    mousedx: i16,
    mousedy: i16,
    hasbeenpredicted: bool,
}
