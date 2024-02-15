use crate::*;

#[allow(non_snake_case, non_camel_case_types, dead_code)]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct VMTWeapon {
    _pad1: [u8;4 * 79],
    pub GetIndex: cfn!(c_int, *const Weapon),
    _pad2: [u8;4 * 318],
    pub GetSlot: cfn!(c_int, *const Weapon),
    _pad3: [u8;4 * 1],
    pub GetName: cfn!(*const c_char, *const Weapon),
    _pad4: [u8;4 * 48],
    pub GetWeaponId: cfn!(c_int, *const Weapon),
    pub GetDamageType: cfn!(c_int, *const Weapon),
    _pad5: [u8;4 * 14],
    pub CalcIsAttackCriticalHelper: cfn!(bool, *const Weapon),
    _pad6: [u8;4 * 28],
    pub CanFireCriticalShot: cfn!(bool, *const Weapon, bool),
    _pad7: [u8;4 * 30],
    pub GetSwingRange: cfn!(c_int, *const Weapon),
}

#[allow(non_snake_case, non_camel_case_types, dead_code)]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Weapon {
    vmt: *const VMTWeapon,
    _pad1: [u8;0x924],
    m_iItemDefinitionIndex: c_int, /* 0x928 */
    _pad2: [u8;0x10C],
    hOwner: CBaseHandle, /* 0xA38 */
    _pad3: [u8;0x10],
    flNextPrimaryAttack: c_float, 
    _pad4: [u8;0x1DC],
    smackTime: c_float, /* 0xC2C, see CTFWeaponBaseMelee::ItemPostFrame() */
    _pad6: [u8;0x10],
    bReadyToBackstab: bool, /* 0xC40 */

}
