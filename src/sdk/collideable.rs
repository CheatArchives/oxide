use crate::math::vector::Vector3;

use super::*;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct VMTCollideable {
    _pad: [u8; 4],
    pub obb_mins_pre_scaled: cfn!(Vector3, &'static Collideable),
    pub obb_maxs_pre_scaled: cfn!(Vector3, &'static Collideable),
    pub obb_mins: cfn!(Vector3, &'static Collideable),
    pub obb_maxs: cfn!(Vector3, &'static Collideable),
}

pub type Collideable = WithVmt<VMTCollideable>;
