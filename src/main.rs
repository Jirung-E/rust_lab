use bytemuck::{Zeroable, NoUninit};

#[repr(u8)]
#[derive(Copy, Clone, Zeroable, NoUninit)]
enum PacketType {
    Raw = 0,
    Move,
    Update,
}


struct Packet {
    size: u16,
    packet_type: PacketType,
    data: Vec<u8>,
}

impl Packet {
    fn new(data: &[u8]) -> Self {
        Self {
            size: data.len() as u16 + 2,
            packet_type: PacketType::Update,
            data: data.to_vec(),
        }
    }

    fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = bytemuck::bytes_of(&self.size).to_vec();
        bytes.extend_from_slice(bytemuck::bytes_of(&self.packet_type));
        bytes.extend_from_slice(&self.data);
        bytes
    }
}


fn main() {
    let packet = Packet::new(&(0..16).collect::<Vec<u8>>());

    let start = std::time::Instant::now();
    for _ in 0..1_000_000 {
        packet.as_bytes();
    }
    println!("{:?}", start.elapsed());  // 266 ms
    println!("{:?}", packet.as_bytes());
    // [18, 0, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]
    // u16, data
    // little endian
}