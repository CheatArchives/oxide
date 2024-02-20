use crate::*;


pub type EntityList = WithVmt<VMTEntityList>;

#[allow(non_snake_case, non_camel_case_types, dead_code)]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VMTEntityList {
    _pad1: [u32; 3],
    pub GetClientEntity: cfn!(*mut Entity, *const EntityList, c_int),
    _pad2: [u32; 4],
    pub GetMaxEntities: cfn!(c_int, *const EntityList),

}
