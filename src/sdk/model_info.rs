use std::intrinsics::size_of;

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
        let ptr = ((self as *const _ as usize
            + self.hitboxindex
            + size_of::<Hitbox>() * id as usize) as *const Hitbox);
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
    group: usize,
    bbmin: Vector3,
    bbmax: Vector3,
    szhitboxnameindex: usize,
    unused: [usize; 8],
}

impl Hitbox {
    pub fn center(&self, bone: &Matrix3x4) -> Vector3 {
        let min = bone.transform(&self.bbmin);
        let max = bone.transform(&self.bbmax);
        Vector3::new(
            (min.x + max.x) / 2.0,
            (min.y + max.y) / 2.0,
            (min.z + max.z) / 2.0,
        )
    }
    pub fn corners(&self, bone: &Matrix3x4) -> [Vector3; 8] {
        let min = bone.transform(&self.bbmin);
        let max = bone.transform(&self.bbmax);
        [
            Vector3::new(min.x, min.y, min.z),
            Vector3::new(max.x, min.y, min.z),
            Vector3::new(min.x, max.y, min.z),
            Vector3::new(min.x, min.y, max.z),
            Vector3::new(max.x, max.y, min.z),
            Vector3::new(min.x, max.y, max.z),
            Vector3::new(max.x, min.y, max.z),
            Vector3::new(min.x, min.y, min.z),
        ]
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
