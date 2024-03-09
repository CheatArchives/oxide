
use std::mem::{size_of};

use crate::*;

pub type ModelInfo = WithVmt<VMTModelInfo>;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct HitboxSet {
    sznameindex: usize,
    numhitboxes: usize,
    hitboxindex: usize,
}

impl HitboxSet {
    pub unsafe fn get_hitbox(&self, id: HitboxId) -> Option<Hitbox> {
        let ptr = (self as *const _ as usize
            + self.hitboxindex
            + size_of::<Hitbox>() * id as usize) as *const Hitbox;
        if ptr.is_null() {
            return None;
        }
        Some(ptr.read_unaligned())
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Hitbox {
    pub bone: usize,
    pub group: usize,
    pub min: Vector3,
    pub max: Vector3,
    pub hitboxnameindex: usize,
    unused: [usize; 8],
}

impl Hitbox {
    pub fn center(&self, ent: &Entity) -> Vector3 {
        let (pos,_) = self.get_bone_pos(ent);
        Vector3::new(
            (self.min.x + self.max.x) / 2.0 + pos.x,
            (self.min.y + self.max.y) / 2.0 + pos.y,
            (self.min.z + self.max.z) / 2.0 + pos.z,
        )
    }
    pub fn get_bone_pos(&self, ent: &Entity) -> (Vector3,Angles) {
         unsafe {
            let mut pos = MaybeUninit::zeroed().assume_init();
            let mut angle = MaybeUninit::zeroed().assume_init();

            (oxide!().get_bone_position_fn)(&ent, self.bone, &mut pos, &mut angle);
            (pos,angle)
        }
    }
    pub fn corners(&self, ent: &Entity) -> [Vector3; 8] {
        let (pos,angle) = self.get_bone_pos(ent);
        let rotation = angle.to_vectors();

        let mut corners = [
            Vector3::zeroed(),
            Vector3::zeroed(),
            Vector3::zeroed(),
            Vector3::zeroed(),
            Vector3::zeroed(),
            Vector3::zeroed(),
            Vector3::zeroed(),
            Vector3::zeroed(),
        ];
        let min = &self.min;
        let max = &self.max;
        for i in 0..8 {
            let x = if i & 0x1 != 0 { max.x } else { min.x };
            let y = if i & 0x2 != 0 { max.y } else { min.y };
            let z = if i & 0x4 != 0 { max.z } else { min.z };

            let mut corner = Vector3::new(x, y, z);

            let mut corner = corner.rotate(&rotation);
            corners[i] = (corner + pos.clone())
        }
        corners
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HitboxId {
    Head,
    Pelvis,
    Spine0,
    Spine1,
    Spine2,
    Spine3,
    LeftUpperArm,
    LeftLowerArm,
    LeftHand,
    RightUpperArm,
    RightLowerArm,
    RightHand,
    LeftHip,
    LeftKnee,
    LeftFoot,
    RightHip,
    RightKnee,
    RightFoot,
}

impl HitboxId {
    pub fn body() -> Vec<HitboxId> {
        (1..=17)
            .map(|x| unsafe { transmute(x) })
            .collect::<Vec<HitboxId>>()
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Model {
    pub handle: &'static c_void,
    pub name: &'static CStr,
    pub load_flags: isize,
    pub server_count: isize,
    pub r#type: isize,
    pub flags: isize,
    pub vec_mins: Vector3,
    pub vec_maxs: Vector3,
    pub radius: f32,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct StudioHdr {
    pub id: usize,
    pub version: usize,
    pub checksum: usize,
    pub name: [c_char; 64],
    pub length: usize,
    pub eyeposition: Vector3,
    pub illumposition: Vector3,
    pub hull_min: Vector3,
    pub hull_max: Vector3,
    pub view_bbmin: Vector3,
    pub view_bbmax: Vector3,
    pub flags: usize,
    pub numbones: usize,
    pub boneindex: usize,
    pub numbonecontrollers: usize,
    pub bonecontrollerindex: usize,
    pub numhitboxsets: usize,
    pub hitboxsetindex: usize,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Bone {
    sznameindex: usize,
    parent: usize,
    bonecontroller: [usize; 6],
    pos: Vector3,
    quat: Vector4,
    rot: Vector3,
    posscale: Vector3,
    rotscale: Vector3,
    pose_to_bone: Matrix3x4,
    alignment: Vector4,
    flags: usize,
    proctype: usize,
    procindex: usize,
    physicsbone: usize,
    surfacepropidx: usize,
    contents: usize,
    unused: [usize; 8],
}
impl StudioHdr {
    pub unsafe fn bone(&self, i: usize) -> Option<&Bone> {
        if i >= self.numbones {
            return None;
        }

        Some(&*(((self as *const _ as usize) + self.boneindex + i) as *const Bone))
    }

    pub unsafe fn get_hitbox_set(&self, i: usize) -> Option<&HitboxSet> {
        if i >= self.numhitboxsets {
            return None;
        }

        Some(
            &*((self as *const _ as usize + self.hitboxsetindex + i * size_of::<HitboxSet>())
                as *const HitboxSet),
        )
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct VMTModelInfo {
    _pad1: [u8; 4 * 3],
    pub get_model_index: cfn!(isize, &'static ModelInfo, &CStr),
    _pad2: [u8; 4 * 25],
    pub get_studio_model: cfn!(*const StudioHdr, *const ModelInfo, *const Model),
}
