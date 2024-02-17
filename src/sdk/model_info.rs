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
    id: c_int,
    version: c_int,
    checksum: c_int,
    /* pszName() */
    name: [c_char; 64],
    length: c_int,
    eyeposition: Vector3,
    illumposition: Vector3,
    hull_min: Vector3,
    hull_max: Vector3,
    view_bbmin: Vector3,
    view_bbmax: Vector3,
    flags: c_int,
    numbones: c_int,
    boneindex: c_int,
    /* pBone(int i) */
    numbonecontrollers: c_int,
    bonecontrollerindex: c_int,
    numhitboxsets: c_int,
    hitboxsetindex: c_int,
    /* ... */
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
