use crate::*;


pub type EntityList = WithVmt<VMTEntityList>;

#[allow(non_snake_case, non_camel_case_types, dead_code)]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct VMTEntityList {
    _pad1: [u8; 4 * 3],
    pub GetClientEntity: cfn!(*const Entity, *const EntityList, c_int),
    _pad2: [u8; 4 * 4],
    pub GetMaxEntities: cfn!(c_int, *const EntityList),
}
