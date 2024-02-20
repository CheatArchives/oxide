use crate::*;

#[allow(non_snake_case, non_camel_case_types, dead_code)]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct UserCmd {
    pub vmt: *const c_void,
    pub command_number: c_int,
    pub tick_count: c_int,
    pub viewangles: Angles,
    pub forwardmove: c_float,
    pub sidemove: c_float,
    pub upmove: c_float,
    pub buttons: Buttons,
    pub impulse: u8,
    pub weaponselect: isize,
    pub weaponsubtype: isize,
    pub random_seed: isize,
    pub mousedx: i16,
    pub mousedy: i16,
    pub hasbeenpredicted: bool,
}

#[allow(non_snake_case, non_camel_case_types, dead_code)]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Buttons {
    pub IN_ATTACK: bool,
    pub IN_JUMP: bool,
    pub IN_DUCK: bool,
    pub IN_FORWARD: bool,
    pub IN_BACK: bool,
    pub IN_USE: bool,
    pub IN_CANCEL: bool,
    pub IN_LEFT: bool,
    pub IN_RIGHT: bool,
    pub IN_MOVELEFT: bool,
    pub IN_MOVERIGHT: bool,
    pub IN_ATTACK2: bool,
    pub IN_RUN: bool,
    pub IN_RELOAD: bool,
    pub IN_ALT1: bool,
    pub IN_ALT2: bool,
    pub IN_SCORE: bool,
    pub IN_SPEED: bool,
    pub IN_WALK: bool,
    pub IN_ZOOM: bool,
    pub IN_WEAPON1: bool,
    pub IN_WEAPON2: bool,
    pub IN_BULLRUSH: bool,
    pub IN_GRENADE1: bool,
    pub IN_GRENADE2: bool,
    pub IN_ATTACK3: bool,
}
impl Buttons {
    fn new() -> Buttons {
        let arr = [
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false
        ];
        unsafe { transmute(arr) }
    }
}
