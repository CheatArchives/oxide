use crate::*;

pub type MaterialSystem = WithVmt<VMTMaterialSystem>;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VMTMaterialSystem {
    _pad1: [u8;4 * 73],
    pub find_material: cfn!(*const IMaterial, *const MaterialSystem , *const c_char, *const c_char, bool, *const c_char),
    _pad2: [u8;4 * 26],
    pub get_render_context: cfn!(*const IMatRenderContext,*const MaterialSystem),
}

pub type IMaterial = WithVmt<VMTIMaterial>;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VMTIMaterial {
    _pad1: [u8;4 * 27],
    pub alpha_modulate: cfn!(c_void, *const IMaterial, c_float),
    pub color_modulate: cfn!(c_void, *const IMaterial, c_float, c_float, c_float),
    pub set_material_var_flag: cfn!(c_void, *const IMaterial, c_int,bool), 
    pub get_material_var_flag: cfn!(bool, *const IMaterial, c_int), 
}

pub type IMatRenderContext = WithVmt<VMTIMatRenderContext>;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VMTIMatRenderContext {
    _pad1: [u8;4 * 11],
    pub depth_range: cfn!(c_void, *const IMatRenderContext, c_float,c_float),
}
