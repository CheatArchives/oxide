use std::usize;

use crate::*;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct UserCmd {
    pub vmt: &'static c_void,
    pub command_number: isize,
    pub tick_count: isize,
    pub viewangles: Angles,
    pub forwardmove: f32,
    pub sidemove: f32,
    pub upmove: f32,
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
        let Buttons(b) = *self;
        b & shifted == shifted
    }
    pub fn set(&mut self, flag: ButtonFlags, val: bool) {
        let flag = flag as u8;
        let mut b: usize = unsafe { transmute(*self) };
        if val {
            b |= 1 << flag;
        } else {
            b &= !(1 << flag);
        }
        *self = unsafe { transmute(b) };
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
