use libc::c_ushort;

use crate::*;

pub type ModelRender = WithVmt<VMTModelRender>;

#[allow(non_snake_case, non_camel_case_types, dead_code)]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Matrix3x4(
    [[c_float;4];3]
);

pub type Renderable = WithVmt<VMTRenderable>;
#[allow(non_snake_case, non_camel_case_types, dead_code)]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VMTRenderable {
    _pad1: [u8;4 * 9],
    pub GetModel: cfn!(&'static Model, *const Renderable),
    _pad2: [u8;4 * 6],
    pub SetupBones: cfn!(bool,*const Renderable, Matrix3x4,c_int,c_int,c_float),
    _pad3: [u8;4 * 17],
    pub RenderableToWorldTransform: cfn!(*mut Matrix3x4,*const Renderable),
}

#[allow(non_snake_case, non_camel_case_types, dead_code)]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ModelRenderInfo{
    origin: Vector3,
    angles: Angles, /* QAngle */
    pRenderable: *const Renderable,
    pModel:*const Model,
    pModelToWorld: *const Matrix3x4,
    pLightingOffset: *const Matrix3x4,
    pLightingOrigin: *const Vector3,
    flags: c_int,
    entity_index: c_int,
    skin: c_int,
    body: c_int,
    hitboxset: c_int,
    instance: c_ushort,
}

#[allow(non_snake_case, non_camel_case_types, dead_code)]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct DrawModelState{
    m_pStudioHdr: *mut StudioHdr ,
    m_pStudioHWData: *mut c_void, /* studiohwdata_t */
    m_pRenderable: *mut Renderable,
    m_pModelToWorld: *const Matrix3x4,
    m_decals: *mut c_void, /* StudioDecalHandle_t */
    m_drawFlags: c_int,
    m_lod: c_int,
} 

#[allow(non_snake_case, non_camel_case_types, dead_code)]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VMTModelRender {
    _pad1: [u8;4 * 1],
    pub ForcedMaterialOverride: cfn!(c_void, *mut ModelRender , *const IMaterial, c_int),
    _pad2: [u8;4 * 17],
    pub DrawModelExecute: cfn!(c_void, *mut ModelRender , *mut DrawModelState, *mut ModelRenderInfo, *mut Matrix3x4),
}
