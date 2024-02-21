use crate::*;

#[allow(non_snake_case, non_camel_case_types, dead_code)]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VMTWeapon {
    _pad1: [u8; 4 * 79],
    pub GetIndex: cfn!(c_int, *const Weapon),
    _pad2: [u8; 4 * 318],
    pub GetSlot: cfn!(c_int, *const Weapon),
    _pad3: [u8; 4 * 1],
    pub GetName: cfn!(*const c_char, *const Weapon),
    _pad4: [u8; 4 * 48],
    pub GetWeaponId: cfn!(c_int, *const Weapon),
    pub GetDamageType: cfn!(c_int, *const Weapon),
    _pad5: [u8; 4 * 14],
    pub CalcIsAttackCriticalHelper: cfn!(bool, *const Weapon),
    _pad6: [u8; 4 * 28],
    pub CanFireCriticalShot: cfn!(bool, *const Weapon, bool),
    _pad7: [u8; 4 * 30],
    pub GetSwingRange: cfn!(c_int, *const Weapon),
}

#[allow(non_snake_case, non_camel_case_types, dead_code)]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Weapon {
    pub vmt: *mut VMTWeapon,
    _pad1: [u8; 0x924],
    pub m_iItemDefinitionIndex: c_int,
    _pad2: [u8; 0x10C],
    pub hOwner: CBaseHandle,
    _pad3: [u8; 0x10],
    pub flNextPrimaryAttack: c_float,
    _pad4: [u8; 0x1DC],
    pub smackTime: c_float,
    _pad6: [u8; 0x10],
    pub bReadyToBackstab: bool,
}
impl_has_vmt!(Weapon, VMTWeapon);
