use crate::*;

pub type MaterialSystem = WithVmt<VMTMaterialSystem>;

#[allow(non_snake_case, non_camel_case_types, dead_code)]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct VMTMaterialSystem {
    _pad1: [u8;4 * 73],
    pub FindMaterial: cfn!(*const IMaterial, *const MaterialSystem , *const c_char, *const c_char, bool, *const c_char),
    _pad2: [u8;4 * 26],
    pub GetRenderContext: cfn!(*const IMatRenderContext,*const MaterialSystem),
}

pub type IMaterial = WithVmt<VMTIMaterial>;

#[allow(non_snake_case, non_camel_case_types, dead_code)]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct VMTIMaterial {
    _pad1: [u8;4 * 27],
    pub AlphaModulate: cfn!(c_void, *const IMaterial, c_float),
    pub ColorModulate: cfn!(c_void, *const IMaterial, c_float, c_float, c_float),
    pub SetMaterialVarFlag: cfn!(c_void, *const IMaterial, c_int,bool), 
    pub GetMaterialVarFlag: cfn!(bool, *const IMaterial, c_int), 
}

pub type IMatRenderContext = WithVmt<VMTIMatRenderContext>;

#[allow(non_snake_case, non_camel_case_types, dead_code)]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct VMTIMatRenderContext {
    _pad1: [u8;4 * 11],
    pub DepthRange: cfn!(c_void, *const IMatRenderContext, c_float,c_float),
}
