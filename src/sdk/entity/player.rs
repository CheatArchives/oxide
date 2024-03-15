use std::intrinsics::transmute_unchecked;

use derivative::Derivative;

use crate::{math::angles::Angles, o, sdk::CBaseHandle};

use super::{condition::Condition, player_class::PlayerClass, user_cmd::UserCmd, Entity};

pub const MAX_WEAPONS: usize = 48;

#[repr(C)]
#[derive(Derivative, Clone, Copy)]
#[derivative(Debug)]
pub struct Player {
    #[derivative(Debug = "ignore")]
    _pad6: [u8; 0xC54],
    pub next_attack: f32,
    #[derivative(Debug = "ignore")]
    _pad7: [u8; 0x84],
    pub my_weapons: [CBaseHandle; MAX_WEAPONS],
    #[derivative(Debug = "ignore")]
    _pad8: [u8; 0xD0],
    pub vec_punch_angle: Angles,
    #[derivative(Debug = "ignore")]
    _pad9: [u8; 0xD0],
    pub object_mode: isize,
    #[derivative(Debug = "ignore")]
    _pad10: [u8; 0x1C4],
    pub angle: Angles,
    #[derivative(Debug = "ignore")]
    _pad11: [u8; 0x48],
    pub current_command: *const UserCmd,
    #[derivative(Debug = "ignore")]
    _pad12: [u8; 0xCC],
    pub tick_base: isize,
    #[derivative(Debug = "ignore")]
    _pad13: [u8; 0x3F8],
    pub player_class: PlayerClass,
    #[derivative(Debug = "ignore")]
    _pad14: [u8; 0x36C],
    pub player_cond: Condition,
    #[derivative(Debug = "ignore")]
    _pad15: [u8; 0x18],
    pub condition_bits: isize,
    #[derivative(Debug = "ignore")]
    _pad16: [u8; 0x418],
    pub allow_move_during_taunt: bool,
    #[derivative(Debug = "ignore")]
    _pad17: [u8; 0x18],
    pub force_taunt_cam: isize,
}

impl Player {
    pub fn as_ent(&self) -> &mut Entity {
        unsafe { transmute_unchecked(self) }
    }
    pub fn can_attack(&self) -> bool {
        let now = o!().global_vars.now();
        self.next_attack <= now
    }
}
