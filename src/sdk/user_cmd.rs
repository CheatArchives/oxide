use crate::*;

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

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Buttons(u32);

impl Buttons {
    pub fn get(&self, flag: ButtonFlags) -> bool {
        let flag = flag as u8;
        let shifted = 1 << flag;
        ButtonFlags::InAttack as u8;
        let Buttons(s) = *self;
        s & shifted == shifted
    }
    pub fn set(&mut self, flag: ButtonFlags, val: bool)  {
        let flag = flag as u8;
        unsafe{
            let s = self as  *mut _ as *mut u32;
            let val = if val {1} else {0};
            *s |= val << flag
        }

    }
}

#[derive(Debug, Clone, Copy)]
pub enum ButtonFlags {
    InAttack,
    InJump,
    InDuck,
    InForward,
    InBack,
    InUse,
    InCancel,
    InLeft,
    InRight,
    InMoveleft,
    InMoveright,
    InAttack2,
    InRun,
    InReload,
    InAlt1,
    InAlt2,
    InScore,
    InSpeed,
    InWalk,
    InZoom,
    InWeapon1,
    InWeapon2,
    InBullrush,
    InGrenade1,
    InGrenade2,
    InAttack3,
}
