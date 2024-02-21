use crate::*;

pub type ModelInfo = WithVmt<VMTModelInfo>;

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

pub struct StudioHdrBone{
     sznameindex: usize, 
     parent: usize, 
     bonecontroller: [usize;6], 
     pos: Vector3, 
     quat: Vector4, 
     rot: Vector3,  /* RadianEuler */
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
     unused: [usize;8], 
}
impl StudioHdr {
 pub unsafe fn get(&self, i:usize) -> Option<StudioHdrBone>{
    if i < 0 || i >= self.numbones{
        return None;
    }

    Some(((self as *const _ as usize) + self.boneindex + i) as StudioHdrBone)
}

static inline studiohitboxset_t* studiohdr_pHitboxSet(studiohdr_t* thisptr,
                                                      int i) {
    if (i < 0 || i >= thisptr->numhitboxsets)
        return NULL;

    return (studiohitboxset_t*)(((void*)thisptr) + thisptr->hitboxsetindex) + i;
}

/* Wrapper for studiohdr_pHitboxSet and studiohitboxset_pHitbox */
static inline studiobbox_t* studiohdr_pHitbox(studiohdr_t* thisptr, int set,
                                              int idx) {
    studiohitboxset_t* hitboxset = studiohdr_pHitboxSet(thisptr, set);
    if (!hitboxset)
        return NULL;

    return (studiobbox_t*)studiohitboxset_pHitbox(hitboxset, idx);
}
pub unsafe fn center_of_hitbox(set: usize,
                        idx:usize) {
    studiobbox_t* bbox = studiohdr_pHitbox(studio, set, idx);
    if (!bbox)
        return VEC_ZERO;

    vec3_t min, max;
    vec_transform(bbox->bbmin, &bonemat[bbox->bone], &min);
    vec_transform(bbox->bbmax, &bonemat[bbox->bone], &max);

    return (vec3_t){
        (min.x + max.x) * 0.5f,
        (min.y + max.y) * 0.5f,
        (min.z + max.z) * 0.5f,
    };
}
}

#[allow(non_snake_case, non_camel_case_types, dead_code)]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VMTModelInfo {
    _pad1: [u8;4 * 3],
    pub GetModelIndex: cfn!(c_int, *const ModelInfo , *const c_char),
    _pad2: [u8;4 * 25],
    pub GetStudioModel: cfn!(StudioHdr, *const ModelInfo , *const Model),
}
