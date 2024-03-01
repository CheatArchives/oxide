use crate::*;


pub type EntityList = WithVmt<VMTEntityList>;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct VMTEntityList {
    _pad1: [u32; 3],
    pub get_client_entity: cfn!(*mut Entity, *const EntityList, isize),
    _pad2: [u32; 4],
    pub get_max_entities: cfn!(isize, *const EntityList),

}

