use crate::*;

#[allow(non_snake_case, non_camel_case_types, dead_code)]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct VMTCollideable {
    _pad: [u8; 4],
    pub ObbMinsPreScaled: cfn!(Vector3, *const Collideable),
    pub ObbMaxsPreScaled: cfn!(Vector3, *const Collideable),
    pub ObbMins: cfn!(Vector3, *const Collideable),
    pub ObbMaxs: cfn!(Vector3, *const Collideable),
}

pub type Collideable = WithVmt<VMTCollideable>;
