use std::f32::consts::PI;

use crate::*;

module_export!(angles);
module_export!(vector);

pub fn dtr(deg: f32) -> f32{
    deg / 180f32 * PI
}
