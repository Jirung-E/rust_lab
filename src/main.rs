struct PacketType(u8);
impl PacketType{
    const RAW: Self = Self(0);
    const MOVE: Self = Self(1);
    const UPDATE: Self = Self(2);

    fn to_be_bytes(&self) -> [u8; 1] {      // size_of::<PacketType>()
        self.0.to_be_bytes()
    }
}

type PacketSize = u16;

struct Packet {
    size: PacketSize,
    packet_type: PacketType,
    data: Vec<u8>,
}

impl Packet {
    fn new(data: &[u8]) -> Self {
        Self {
            size: (data.len() + std::mem::size_of::<PacketSize>()) as PacketSize,
            packet_type: PacketType::RAW,
            data: data.to_vec(),
        }
    }

    fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(self.size as usize);
        bytes.extend_from_slice(&self.size.to_be_bytes());
        bytes.extend_from_slice(&self.packet_type.to_be_bytes());
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
    // [0, 18, 0, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]
    // u16, type, data
    // big-endian
}