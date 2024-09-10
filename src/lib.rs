use bytemuck::{Pod, Zeroable};


#[repr(C)]
#[derive(Debug, PartialEq, Clone, Copy, Pod, Zeroable)]
pub struct Player {
    pub id: u32,
    pub translation: gmm::Float3, 
    pub rotation: gmm::Float4, 
    pub anim_index: u32, 
    pub anim_timer: f32, 
}

impl Player {
    pub fn new(
        id: u32, 
        translation: gmm::Float3, 
        rotation: gmm::Float4, 
        anim_index: u32, 
        anim_timer: f32, 
    ) -> Self {
        Self {
            id, 
            translation, 
            rotation, 
            anim_index, 
            anim_timer, 
        }
    }
    
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = bytemuck::bytes_of(self).to_vec();
        bytes[0..4].reverse();
        bytes[4..8].reverse();
        bytes[8..12].reverse();
        bytes[12..16].reverse();
        bytes[16..20].reverse();
        bytes[20..24].reverse();
        bytes[24..28].reverse();
        bytes[28..32].reverse();
        bytes[32..36].reverse();
        bytes[36..40].reverse();
        bytes
    }

    pub fn from_bytes(data: &[u8]) -> Player {
        *bytemuck::from_bytes(data)
    }
}

impl Default for Player {
    #[inline]
    fn default() -> Self {
        Self { 
            id: 0, 
            translation: gmm::Float3::ZERO, 
            rotation: gmm::Float4::W, 
            anim_index: 0, 
            anim_timer: 0.0 
        }
    }
}