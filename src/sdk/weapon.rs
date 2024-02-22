use crate::*;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VMTWeapon {
    _pad1: [u8; 4 * 79],
    pub get_index: cfn!(isize, &'static Weapon),
    _pad2: [u8; 4 * 318],
    pub get_slot: cfn!(isize, &'static Weapon),
    _pad3: [u8; 4 * 1],
    pub get_name: cfn!(&'static CStr, &'static Weapon),
    _pad4: [u8; 4 * 48],
    pub get_weapon_id: cfn!(isize, &'static Weapon),
    pub get_damage_type: cfn!(isize, &'static Weapon),
    _pad5: [u8; 4 * 14],
    pub calc_is_attack_critical_helper: cfn!(bool, &'static Weapon),
    _pad6: [u8; 4 * 28],
    pub can_fire_critical_shot: cfn!(bool, &'static Weapon, bool),
    _pad7: [u8; 4 * 30],
    pub get_swing_range: cfn!(isize, &'static Weapon),
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Weapon {
    pub vmt: &'static VMTWeapon,
    _pad1: [u8; 0x924],
    pub item_definition_index: isize,
    _pad2: [u8; 0x10C],
    pub owner: CBaseHandle,
    _pad3: [u8; 0x10],
    pub next_primary_attack: f32,
    _pad4: [u8; 0x1DC],
    pub smack_time: f32,
    _pad6: [u8; 0x10],
    pub ready_to_backstab: bool,
}
impl_has_vmt!(Weapon, VMTWeapon);
