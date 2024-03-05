use crate::*;


pub type EntityList = WithVmt<VMTEntityList>;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct VMTEntityList {
    _pad1: [u32; 3],
    pub get_client_entity: cfn!(*mut Entity, *const EntityList, isize),
	pub get_client_entity_from_handle: cfn!(&'static Entity, *const EntityList, CBaseHandle ),
	pub number_of_entities: cfn!(i32, *const EntityList, bool ),
	pub get_highest_entity_index: cfn!(i32, *const EntityList),
	pub set_max_entities: cfn!((), *const EntityList),
    pub get_max_entities: cfn!(isize, *const EntityList),

}

