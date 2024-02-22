use crate::*;

pub type EngineTrace = WithVmt<VMTEngineTrace>;


#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Ray {
    start: Vector3,
    delta: Vector3,
    start_offset: Vector3,
    extents: Vector3,
    is_ray: bool,
    is_swept: bool,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VMTEngineTrace {
    _pad1: [u8;4 * 4],
    pub trace_ray: cfn!(isize, &'static EngineTrace , &'static Ray, usize),
}
