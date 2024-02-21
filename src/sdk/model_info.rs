use libc::wait;

use crate::*;

pub type ModelInfo = WithVmt<VMTModelInfo>;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HitboxSet {
    sznameindex: usize,
    numhitboxes: usize,
    hitboxindex: usize,
}

impl HitboxSet {
    ///* Wrapper for studiohdr_pHitboxSet and studiohitboxset_pHitbox */
    //static inline studiobbox_t* studiohitboxset_pHitbox(studiohitboxset_t* thisptr,
    //                                                    int i) {
    //    return (studiobbox_t*)(((void*)thisptr) + thisptr->hitboxindex) + i;
    //};
    pub unsafe fn get_box(&self, id: HitboxId) -> Option<&Hitbox> {
        let addr = self as *const _ as usize + self.hitboxindex + id as usize;
        let ptr = transmute::<usize, *const Hitbox>(addr);
        if ptr.is_null() {
            return None;
        }
        Some(&*ptr)
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Hitbox {
    bone: usize,
    group: usize,
    bbmin: Vector3,
    bbmax: Vector3,
    szhitboxnameindex: usize,
    unused: [usize; 8],
}

impl Hitbox {
    pub unsafe fn center(&self, bones: [Matrix3x4; MAX_STUDIO_BONES]) -> Vector3 {

        let bone = bones[self.bone];
        let min = bone.transform(self.bbmin);
        let max = bone.transform(self.bbmax);
        Vector3 {
            x: (min.x + max.x) / 2.0,
            y: (min.y + max.y) / 2.0,
            z: (min.z + max.z) / 2.0,
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum HitboxId {
    HitboxHead,
    HitboxPelvis,
    HitboxSpine0,
    HitboxSpine1,
    HitboxSpine2,
    HitboxSpine3,
    HitboxLeftUpperArm,
    HitboxLeftLowerArm,
    HitboxLeftHand,
    HitboxRightUpperArm,
    HitboxRightLowerArm,
    HitboxRightHand,
    HitboxLeftHip,
    HitboxLeftKnee,
    HitboxLeftFoot,
    HitboxRightHip,
    HitboxRightKnee,
    HitboxRightFoot,
    HitboxMax,
}

#[allow(non_snake_case, non_camel_case_types, dead_code)]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Model {
    handle: *const c_void,
    name: *const c_char,
    load_flags: c_int,
    server_count: c_int,
    r#type: c_int,
    flags: c_int,
    vec_mins: Vector3,
    vec_maxs: Vector3,
    radius: c_float,
}

#[allow(non_snake_case, non_camel_case_types, dead_code)]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
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
#[derive(Debug, Clone, Copy)]
pub struct Bone {
    sznameindex: usize,
    parent: usize,
    bonecontroller: [usize; 6],
    pos: Vector3,
    quat: Vector4,
    rot: Vector3, /* RadianEuler */
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
        if i < 0 || i >= self.numbones {
            return None;
        }

        Some(&*(((self as *const _ as usize) + self.boneindex + i) as *const Bone))
    }

    pub unsafe fn hitbox_set(&self, i: usize) -> Option<&HitboxSet> {
        if i < 0 || i >= self.numhitboxsets {
            return None;
        }

        Some(&*(((self as *const _ as usize) + self.hitboxsetindex + i) as *const HitboxSet))
    }
}

#[allow(non_snake_case, non_camel_case_types, dead_code)]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VMTModelInfo {
    _pad1: [u8; 4 * 3],
    pub GetModelIndex: cfn!(c_int, *const ModelInfo, *const c_char),
    _pad2: [u8; 4 * 25],
    pub GetStudioModel: cfn!(&'static StudioHdr, *const ModelInfo, *const Model),
}
