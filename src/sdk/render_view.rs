use crate::*;

pub type RenderView = WithVmt<VMTRenderView>;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct FloatRGBA(c_float, c_float, c_float, c_float);

#[allow(non_snake_case, non_camel_case_types, dead_code)]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct VMTRenderView {
    _pad1: [u32; 4],
    pub SetBlend: cfn!(c_void, *const RenderView, c_float),
    pub GetBlend: cfn!(c_float, *const RenderView),
    pub SetColorModulation: cfn!(c_void, *const RenderView, *const FloatRGBA),
    pub GetColorModulation: cfn!(c_void, *const RenderView, *mut FloatRGBA),
    _pad2: [u32; 42],
    pub GetMatricesForView: cfn!(
        c_void,
        *const RenderView,
        *const VMatrix,
        *const VMatrix,
        *const VMatrix,
        *const VMatrix
    ),
}
