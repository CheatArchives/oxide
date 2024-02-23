use std::mem::MaybeUninit;

use crate::*;

pub const MAX_WEAPONS: usize = 48;
pub const MAX_STUDIO_BONES: usize = 128;
pub const HITBOX_SET: usize = 0;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum BoneMask {
    BoneUsedByAnything = 0x0007FF00,
    BoneUsedByHitbox = 0x00000100,
    BoneUsedByAttachment = 0x00000200,
    BoneUsedByVertexMask = 0x0003FC00,
    BoneUsedByVertexLod0 = 0x00000400,
    BoneUsedByVertexLod1 = 0x00000800,
    BoneUsedByVertexLod2 = 0x00001000,
    BoneUsedByVertexLod3 = 0x00002000,
    BoneUsedByVertexLod4 = 0x00004000,
    BoneUsedByVertexLod5 = 0x00008000,
    BoneUsedByVertexLod6 = 0x00010000,
    BoneUsedByVertexLod7 = 0x00020000,
    BoneUsedByBoneMerge = 0x00040000,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VMTEntity {
    _pad1: [u32; 4],
    pub get_collideable: cfn!(&Collideable, &Entity),
    _pad2: [u32; 6],
    pub get_abs_origin: cfn!(&Vector3, &Entity),
    pub get_abs_angles: cfn!(&mut Angles, &Entity),
    _pad3: [u32; 66],
    pub get_index: cfn!(&isize, &Entity),
    _pad4: [u32; 26],
    pub world_space_center: cfn!(&Vector3, &Entity),
    _pad5: [u32; 10],
    pub get_team_number: cfn!(isize, &Entity),
    _pad6: [u32; 34],
    pub get_health: cfn!(&isize, &Entity),
    pub get_max_health: cfn!(&isize, &Entity),
    _pad7: [u32; 29],
    pub is_alive: cfn!(bool, &Entity),
    pub is_player: cfn!(bool, &Entity),
    _pad8: [u32; 2],
    pub is_npc: cfn!(bool, &Entity),
    _pad9: [u32; 2],
    pub is_weapon: cfn!(bool, &Entity),
    _pad10: [u32; 3],
    pub eye_position: cfn!(Vector3, &Entity),
    pub eye_angles: cfn!(&Vector3, &Entity),
    _pad11: [u32; 12],
    pub third_person_switch: cfn!(c_void, &Entity, bool),
    _pad12: [u32; 82],
    pub get_weapon: cfn!(&mut Weapon, &Entity),
    _pad13: [u32; 10],
    pub get_shoot_pos: cfn!(Vector3, &Entity),
    _pad14: [u32; 6],
    pub get_observer_mode: cfn!(isize, &Entity),
    pub get_observer_target: cfn!(&Entity, &Entity),
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Entity {
    pub vmt: &'static VMTEntity,
    _pad1: [u8; 0x7C],
    pub model_idx: isize,
    _pad2: [u8; 0x8C],
    pub velocity: Vector3,
    _pad3: [u8; 0x7C],
    pub water_level: usize,
    _pad4: [u8; 0x1B8],
    pub vec_origin: Vector3,
    _pad5: [u8; 0xC],
    pub flags: isize,
    _pad6: [u8; 0x8E4],
    pub next_attack: f32,
    _pad7: [u8; 0x84],
    pub my_weapons: [CBaseHandle; MAX_WEAPONS],
    _pad8: [u8; 0xD0],
    pub vec_punch_angle: Angles,
    _pad9: [u8; 0xD0],
    pub object_mode: isize,
    _pad10: [u8; 0x1C4],
    pub angle: Angles,
    _pad11: [u8; 0x48],
    pub current_command: *const UserCmd,
    _pad12: [u8; 0xCC],
    pub tick_base: isize,
    _pad13: [u8; 0x3F8],
    pub player_class: isize,
    _pad14: [u8; 0x36C],
    pub player_cond: Condition,
    _pad15: [u8; 0x18],
    pub condition_bits: isize,
    _pad16: [u8; 0x418],
    pub allow_move_during_taunt: bool,
    _pad17: [u8; 0x18],
    pub force_taunt_cam: isize,
}

impl_has_vmt!(Entity, VMTEntity);

impl Entity {
    pub unsafe fn get(id: isize) -> Option<&'static mut Entity> {
        let Some(mut ent) = get_entity(id) else {
            return None;
        };
        let net = ent.networkabe();

        {
            if call!(net, is_dormant) || !call!(ent, is_alive) || !call!(ent, is_player) {
                return None;
            }
        }

        Some(transmute(addr_of_mut!(ent)))
    }

    pub unsafe fn can_attack(&mut self) -> bool {
        let now = oxide!().global_vars.interval_per_tick * self.tick_base as f32;
        if !call!(self, is_alive) {
            return false;
        }
        let weapon = call!(self, get_weapon);
        self.next_attack <= now && weapon.next_primary_attack <= now
    }
    pub unsafe fn networkabe(&self) -> Networkable {
        *((self as *const Entity as usize + 0x8) as *mut c_void as *mut _ as *mut Networkable)
    }

    pub unsafe fn renderable(&self) -> Renderable {
        *((self as *const Entity as usize + 0x4) as *mut c_void as *mut _ as *mut Renderable)
    }

    pub unsafe fn get_hitbox(&self, hitbox_id: HitboxId) -> Option<Vector3> {
        let bones: [Matrix3x4; MAX_STUDIO_BONES] = MaybeUninit::zeroed().assume_init();
        let rend = self.renderable();
        if !call!(
            rend,
            setup_bones,
            &bones,
            MAX_STUDIO_BONES,
            BoneMask::BoneUsedByHitbox,
            0f32
        ) {
            return None;
        }
        let model = call!(rend, get_model);
        let hdr = call!(interface!(model_info), get_studio_model, model);
        let Some(hitbox_set) = hdr.hitbox_set(HITBOX_SET) else {
            return None;
        };
        let Some(hitbox) = hitbox_set.get_hitbox(hitbox_id) else {
            return None;
        };
        Some(hitbox.center(bones))
    }
}
