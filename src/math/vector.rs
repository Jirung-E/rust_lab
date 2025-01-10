#[derive(Debug, Clone, Copy)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub fn new() -> Vector3 {
        Vector3 { 
            x: 0.0, 
            y: 0.0, 
            z: 0.0
         }
    }

    pub fn from(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3 { x, y, z }
    }

    pub fn random() -> Vector3 {
        Vector3 {
            x: rand::random(),
            y: rand::random(),
            z: rand::random(),
        }
    }
}

impl From<(f32, f32, f32)> for Vector3 {
    fn from((x, y, z): (f32, f32, f32)) -> Vector3 {
        Vector3 { x, y, z }
    }
}

impl From<[f32; 3]> for Vector3 {
    fn from([x, y, z]: [f32; 3]) -> Vector3 {
        Vector3 { x, y, z }
    }
}
