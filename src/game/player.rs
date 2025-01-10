use crate::math::Vector3;


#[derive(Debug, Clone)]
pub struct Player {
    pub name: String,
    pub hp: u32,
    pub mp: u32,
    pub position: Vector3,
}

impl Player {
    pub fn new(name: &str) -> Player {
        Player {
            name: name.to_string(),
            hp: 100,
            mp: 100,
            position: Vector3::new(),
        }
    }
}