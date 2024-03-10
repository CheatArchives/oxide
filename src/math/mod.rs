use std::f32::consts::PI;

pub mod angles;
pub mod vector;

pub fn dtr(deg: f32) -> f32 {
    deg / 180f32 * PI
}
