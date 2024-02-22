use crate::*;


pub type EntityList = WithVmt<VMTEntityList>;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VMTEntityList {
    _pad1: [u32; 3],
    pub get_client_entity: cfn!(*mut Entity, *const EntityList, c_int),
    _pad2: [u32; 4],
    pub get_max_entities: cfn!(c_int, *const EntityList),

}
