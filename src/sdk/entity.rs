use crate::*;

const MAX_WEAPONS: usize = 48;

#[allow(non_snake_case, non_camel_case_types, dead_code)]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VMTEntity {
    _pad1: [u32; 4],
    pub GetCollideable: cfn!(*const Collideable, *const Entity),
    _pad2: [u32; 6],
    pub GetAbsOrigin: cfn!(*const Vector3, *const Entity),
    pub GetAbsAngles: cfn!(&'static mut Angles, *const Entity),
    _pad3: [u32; 66],
    pub GetIndex: cfn!(*const c_int, *const Entity),
    _pad4: [u32; 26],
    pub WorldSpaceCenter: cfn!(*const Vector3, *const Entity),
    _pad5: [u32; 10],
    pub GetTeamNumber: cfn!(isize, *const Entity),
    _pad6: [u32; 34],
    pub GetHealth: cfn!(*const c_int, *const Entity),
    pub GetMaxHealth: cfn!(*const c_int, *const Entity),
    _pad7: [u32; 29],
    pub IsAlive: cfn!(bool, *const Entity),
    pub IsPlayer: cfn!(bool, *const Entity),
    _pad8: [u32; 2],
    pub IsNPC: cfn!(bool, *const Entity),
    pub IsWeapon: cfn!(bool, *const Entity),
    _pad9: [u32; 3],
    pub EyePosition: cfn!(*const Vector3, *const Entity),
    pub EyeAngles: cfn!(*const Vector3, *const Entity),
    _pad10: [u32; 12],
    pub ThirdPersonSwitch: cfn!(c_void, *const Entity, bool),
    _pad11: [u32; 82],
    pub GetWeapon: cfn!(*const Weapon, *const Entity),
    _pad12: [u32; 10],
    pub GetShootPos: cfn!(Vector3, *const Entity),
    _pad13: [u32; 6],
    pub GetObserverMode: cfn!(c_int, *const Entity),
    pub GetObserverTarget: cfn!(*const Entity, *const Entity),
}

#[allow(non_snake_case, non_camel_case_types, dead_code)]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Entity {
    pub vmt: *mut VMTEntity,
    _pad1: [u8; 0x7C],
    pub model_idx: c_int, /* 0x80 */
    _pad2: [u8; 0x8C],
    pub velocity: Vector3, /* 0x110 */
    _pad3: [u8; 0x7C],
    pub m_nWaterLevel: c_uint, /* 0x198, gcc adds 3 bytes of padding */
    _pad4: [u8; 0x1B8],
    pub m_vecOrigin: Vector3, /* 0x354 */
    _pad5: [u8; 0xC],
    pub flags: c_int, /* 0x36C */
    _pad6: [u8; 0x8E4],
    pub flNextAttack: c_float, /* 0xC54 */
    _pad7: [u8; 0x84],
    pub m_hMyWeapons: [CBaseHandle; MAX_WEAPONS], /* 0xCDC */
    _pad8: [u8; 0xD0],                        /* Starts at 0xD9C */
    pub vecPunchAngle: Angles,                      /* 0xE6C */
    _pad9: [u8; 0xD0],
    pub m_iObjectMode: c_int, /* 0xF48 */
    _pad10: [u8; 0x1C4],
    pub v_angle: Angles,
    _pad11: [u8; 0x48],
    pub m_pCurrentCommand: *const UserCmd, /* 0x1164, see CPrediction::StartCommand() */
    _pad12: [u8; 0xCC],
    pub nTickBase: c_int, /* 0x1234 */
    _pad13: [u8; 0x3F8],
    pub player_class: c_int, /* 0x1630 (ETFClass) */
    _pad14: [u8; 0x36C],
    pub m_nPlayerCond: c_int,    /* 0x19A0 */
    pub m_nPlayerCondEx: c_int,  /* 0x19A4 */
    pub m_nPlayerCondEx2: c_int, /* 0x19A8 */
    pub m_nPlayerCondEx3: c_int, /* 0x19AC */
    pub m_nPlayerCondEx4: c_int, /* 0x19B0 */
    _pad15: [u8; 0x18],
    pub condition_bits: c_int, /* 0x19CC */
    _pad16: [u8; 0x418],
    pub m_bAllowMoveDuringTaunt: bool, /* 0x1DE8 */
    _pad17: [u8; 0x18],
    pub nForceTauntCam: c_int, /* 0x1E04 */
}

impl_has_vmt!(Entity,VMTEntity);

