use std::ops::Sub;

use crate::*;

#[allow(non_snake_case, non_camel_case_types, dead_code)]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Vector3 {
    pub x: c_float,
    pub y: c_float,
    pub z: c_float,
}

impl Sub for Vector3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let Vector3 {
            x: lx,
            y: ly,
            z: lz,
        } = self;
        let Vector3 {
            x: rx,
            y: ry,
            z: rz,
        } = rhs;
        Vector3::new(lx - rx, ly - ry, rz - lz)
    }
}

impl Vector3 {
    pub fn new(x:f32,y:f32,z:f32) -> Vector3 {
        Vector3{x,y,z}
    }

}
