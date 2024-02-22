use crate::*;

pub type RenderView = WithVmt<VMTRenderView>;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FloatRGBA(c_float, c_float, c_float, c_float);

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VMTRenderView {
    _pad1: [u32; 4],
    pub set_blend: cfn!(c_void, *const RenderView, c_float),
    pub get_blend: cfn!(c_float, *const RenderView),
    pub set_color_modulation: cfn!(c_void, *const RenderView, *const FloatRGBA),
    pub get_color_modulation: cfn!(c_void, *const RenderView, *mut FloatRGBA),
    _pad2: [u32; 42],
    pub get_matrices_for_view: cfn!(
        c_void,
        *const RenderView,
        *const VMatrix,
        *const VMatrix,
        *const VMatrix,
        *const VMatrix
    ),
}
