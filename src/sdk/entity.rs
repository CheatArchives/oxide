use std::mem::MaybeUninit;

use crate::*;

pub const MAX_WEAPONS: usize = 48;
pub const MAX_STUDIO_BONES: usize = 128;
pub const HITBOX_SET: usize = 0;

#[allow(non_snake_case, non_camel_case_types, dead_code)]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum BoneMask {
    BONE_USED_BY_ANYTHING = 0x0007FF00,
    BONE_USED_BY_HITBOX = 0x00000100,
    BONE_USED_BY_ATTACHMENT = 0x00000200,
    BONE_USED_BY_VERTEX_MASK = 0x0003FC00,
    BONE_USED_BY_VERTEX_LOD0 = 0x00000400,
    BONE_USED_BY_VERTEX_LOD1 = 0x00000800,
    BONE_USED_BY_VERTEX_LOD2 = 0x00001000,
    BONE_USED_BY_VERTEX_LOD3 = 0x00002000,
    BONE_USED_BY_VERTEX_LOD4 = 0x00004000,
    BONE_USED_BY_VERTEX_LOD5 = 0x00008000,
    BONE_USED_BY_VERTEX_LOD6 = 0x00010000,
    BONE_USED_BY_VERTEX_LOD7 = 0x00020000,
    BONE_USED_BY_BONE_MERGE = 0x00040000,
}

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
    _pad9: [u32; 2],
    pub IsWeapon: cfn!(bool, *const Entity),
    _pad10: [u32; 3],
    pub EyePosition: cfn!(Vector3, *const Entity),
    pub EyeAngles: cfn!(*const Vector3, *const Entity),
    _pad11: [u32; 12],
    pub ThirdPersonSwitch: cfn!(c_void, *const Entity, bool),
    _pad12: [u32; 82],
    pub GetWeapon: cfn!(&'static mut Weapon, *const Entity),
    _pad13: [u32; 10],
    pub GetShootPos: cfn!(Vector3, *const Entity),
    _pad14: [u32; 6],
    pub GetObserverMode: cfn!(c_int, *const Entity),
    pub GetObserverTarget: cfn!(*const Entity, *const Entity),
}

#[allow(non_snake_case, non_camel_case_types, dead_code)]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Entity {
    pub vmt: *mut VMTEntity,
    _pad1: [u8; 0x7C],
    pub model_idx: c_int,
    _pad2: [u8; 0x8C],
    pub velocity: Vector3,
    _pad3: [u8; 0x7C],
    pub m_nWaterLevel: c_uint,
    _pad4: [u8; 0x1B8],
    pub m_vecOrigin: Vector3,
    _pad5: [u8; 0xC],
    pub flags: c_int,
    _pad6: [u8; 0x8E4],
    pub flNextAttack: c_float,
    _pad7: [u8; 0x84],
    pub m_hMyWeapons: [CBaseHandle; MAX_WEAPONS],
    _pad8: [u8; 0xD0],
    pub vecPunchAngle: Angles,
    _pad9: [u8; 0xD0],
    pub m_iObjectMode: c_int,
    _pad10: [u8; 0x1C4],
    pub v_angle: Angles,
    _pad11: [u8; 0x48],
    pub m_pCurrentCommand: *const UserCmd,
    _pad12: [u8; 0xCC],
    pub nTickBase: c_int,
    _pad13: [u8; 0x3F8],
    pub player_class: c_int,
    _pad14: [u8; 0x36C],
    pub m_nPlayerCond: Condition,
    _pad15: [u8; 0x18],
    pub condition_bits: isize,
    _pad16: [u8; 0x418],
    pub m_bAllowMoveDuringTaunt: bool,
    _pad17: [u8; 0x18],
    pub nForceTauntCam: c_int,
}

impl_has_vmt!(Entity, VMTEntity);

impl Entity {
    pub unsafe fn get(id: i32) -> Option<&'static mut Entity> {
        let ent_ptr = call!(interface_ref!(entity_list), GetClientEntity, id);
        if ent_ptr.is_null() {
            return None;
        }
        let ent = &mut *ent_ptr;
        let net = ent.networkabe();

        if ent_ptr.is_null()
            || call!(net, IsDormant)
            || !call!(ent, IsAlive)
            || !call!(ent, IsPlayer)
        {
            return None;
        }

        Some(ent)
    }

    pub unsafe fn can_attack(&mut self) -> bool {
        let now = o!().global_vars.interval_per_tick * self.nTickBase as f32;
        if !call!(self, IsAlive) {
            return false;
        }
        let weapon = call!(self, GetWeapon);
        self.flNextAttack <= now && weapon.flNextPrimaryAttack <= now
    }
    pub unsafe fn networkabe(&self) -> &'static mut Networkable {
        &mut *((self as *const Entity as usize + 0x8) as *mut c_void as *mut _ as *mut Networkable)
    }

    pub unsafe fn renderable(&self) -> &'static mut Renderable {
        &mut *((self as *const Entity as usize + 0x4) as *mut c_void as *mut _ as *mut Renderable)
    }

    pub unsafe fn get_hitbox(&self, hitbox_id: HitboxId) -> Option<Vector3> {
        let bones: [Matrix3x4; MAX_STUDIO_BONES] = MaybeUninit::zeroed().assume_init();
        let rend = self.renderable();
        if !call!(
            rend,
            SetupBones,
            &bones,
            MAX_STUDIO_BONES,
            BoneMask::BONE_USED_BY_HITBOX,
            0f32
        ) {
            return None;
        }
        let model = call!(rend, GetModel);
        let hdr = call_interface!(model_info, GetStudioModel, model);
        let Some(hitbox_set) = hdr.hitbox_set(HITBOX_SET) else {
            return None;
        };
        let Some(hitbox) = hitbox_set.get_hitbox(hitbox_id) else {
            return None;
        };
        Some(hitbox.center(bones))
    }

}
