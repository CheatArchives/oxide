use std::{f32::consts::PI, ops::Sub};

use crate::{Angles, VectorAligned};

#[derive(Debug, Clone)]
#[repr(C)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3 { x, y, z }
    }
    pub fn dot(&self, vec: Vector3) -> f32 {
        self.x * vec.x + self.y * vec.y + self.z * vec.z
    }
    pub fn len2d(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
    pub fn len3d(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }
    pub fn angle(&self) -> Angles {
        Angles {
            pitch: self.z.atan2(self.len2d()) / PI * 180f32,
            yaw: self.y.atan2(self.x) / PI * 180f32 + 180f32,
            roll: 0.0,
        }
    }
    pub fn empty() -> Vector3 {
        Vector3::new(0.0, 0.0, 0.0)
    }
}

impl Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Default for Vector3 {
    fn default() -> Self {
        Vector3::new(0f32, 0f32, 0f32)
    }
}

impl Into<VectorAligned> for Vector3 {
    fn into(self) -> VectorAligned {
        VectorAligned::new(self.x, self.y, self.z)
    }
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct Vector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub t: f32,
}
