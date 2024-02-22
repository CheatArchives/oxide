use libc::c_ushort;

use crate::*;

pub type ModelRender = WithVmt<VMTModelRender>;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Matrix3x4([[f32; 4]; 3]);

impl Matrix3x4 {
    pub unsafe fn transform(&self, vec: Vector3) -> Vector3 {
        let matrix = self.0;
        let vec1 = Vector3::new(matrix[0][0], matrix[0][1], matrix[0][2]);
        let vec2 = Vector3::new(matrix[1][0], matrix[1][1], matrix[1][2]);
        let vec3 = Vector3::new(matrix[2][0], matrix[2][1], matrix[2][2]);
        Vector3 {
            x: vec.dot(vec1) + matrix[0][3],
            y: vec.dot(vec2) + matrix[1][3],
            z: vec.dot(vec3) + matrix[2][3],
        }
    }
}

pub type Renderable = WithVmt<VMTRenderable>;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VMTRenderable {
    _pad1: [u8; 4 * 9],
    pub get_model: cfn!(&Model, &Renderable),
    _pad2: [u8; 4 * 6],
    pub setup_bones: cfn!(
        bool,
        &Renderable,
        &[Matrix3x4; MAX_STUDIO_BONES],
        usize,
        BoneMask,
        f32
    ),
    _pad3: [u8; 4 * 17],
    pub renderable_to_world_transform: cfn!(&mut Matrix3x4, &'static Renderable),
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ModelRenderInfo {
    origin: Vector3,
    angles: Angles,
    renderable: &'static Renderable,
    model: &'static Model,
    model_to_world: &'static Matrix3x4,
    lighting_offset: &'static Matrix3x4,
    lighting_origin: &'static Vector3,
    flags: isize,
    entity_index: isize,
    skin: isize,
    body: isize,
    hitboxset: isize,
    instance: c_ushort,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct DrawModelState {
    studio_hdr:  *mut StudioHdr,
    studio_hw_data: *mut c_void,
    renderable:  *mut Renderable,
    model_to_world: &'static Matrix3x4,
    decals:  *mut c_void,
    draw_flags: isize,
    lod: isize,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VMTModelRender {
    _pad1: [u8; 4 * 1],
    pub forced_material_override: cfn!(c_void, &'static mut ModelRender, &'static IMaterial, isize),
    _pad2: [u8; 4 * 17],
    pub draw_model_execute: cfn!(
        c_void,
        &'static mut ModelRender,
        &'static mut DrawModelState,
        &'static mut ModelRenderInfo,
        &'static mut Matrix3x4
    ),
}
