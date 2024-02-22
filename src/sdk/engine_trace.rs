use crate::*;

pub type EngineTrace = WithVmt<VMTEngineTrace>;

#[repr(C,align(16))]
#[derive(Debug, Clone, Copy)]
pub struct VectorAligned(c_float, c_float, c_float);

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Ray {
    start: VectorAligned,
    delta: VectorAligned,
    start_offset: VectorAligned,
    extents: VectorAligned,
    is_ray: bool,
    is_swept: bool,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VMTEngineTrace {
    _pad1: [u8;4 * 4],
    pub trace_ray: cfn!(c_int, *const EngineTrace , *const Ray, c_uint),
}
