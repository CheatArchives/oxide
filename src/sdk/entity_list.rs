use crate::*;


pub type EntityList = WithVmt<VMTEntityList>;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VMTEntityList {
    _pad1: [u32; 3],
    pub get_client_entity: cfn!(&'static mut Entity, &'static EntityList, isize),
    _pad2: [u32; 4],
    pub get_max_entities: cfn!(isize, &'static EntityList),

}
