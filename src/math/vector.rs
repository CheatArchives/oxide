use std::ops::Sub;

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Vector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub t: f32,
}
