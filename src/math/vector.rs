use std::ops::Sub;


#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    fn new(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3 { x, y, z }
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
