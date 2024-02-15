use crate::*;

pub type EngineTrace = WithVmt<VMTEngineTrace>;

#[allow(non_snake_case, non_camel_case_types, dead_code)]
#[repr(C,align(16))]
#[derive(Debug, Clone)]
pub struct VectorAligned(c_float, c_float, c_float);

#[allow(non_snake_case, non_camel_case_types, dead_code)]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Ray {
    m_Start: VectorAligned,
    m_Delta: VectorAligned,
    m_StartOffset: VectorAligned,
    m_Extents: VectorAligned,
    m_IsRay: bool,
    m_IsSwept: bool,
}

#[allow(non_snake_case, non_camel_case_types, dead_code)]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct VMTEngineTrace {
    _pad1: [u8;4 * 4],
    pub TraceRay: cfn!(c_int, *const EngineTrace , *const Ray, c_uint),
}
