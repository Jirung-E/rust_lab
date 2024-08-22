use std::mem::size_of;

struct PacketType(u8);
impl PacketType{
    const RAW: Self = Self(0);
    const MOVE: Self = Self(1);
    const UPDATE: Self = Self(2);

    // 크기가 u8보다 커지면 이 함수 활성화
    // fn to_be_bytes(&self) -> [u8; size_of::<PacketType>()] {
    //     self.0.to_be_bytes()
    // }
}

type PacketSize = u16;

#[repr(packed)]
struct PacketHeader {
    size: PacketSize,
    packet_type: PacketType,
}

impl PacketHeader {
    fn to_be_bytes(&self) -> [u8; size_of::<PacketHeader>()] {
        let mut bytes = [0; size_of::<PacketHeader>()];
        bytes[..size_of::<PacketSize>()].copy_from_slice(&self.size.to_be_bytes());
        bytes[size_of::<PacketSize>()] = self.packet_type.0;
        // bytes[size_of::<PacketSize>()..].copy_from_slice(&self.packet_type.to_be_bytes());   // 크기가 u8보다 커지면 이 코드 활성화
        bytes
    }
}

struct Packet {
    header: PacketHeader,
    data: Vec<u8>,
}

impl Packet {
    fn new(data: &[u8]) -> Self {
        Self {
            header: PacketHeader {
                size: (data.len() + std::mem::size_of::<PacketSize>()) as PacketSize,
                packet_type: PacketType::RAW,
            },
            data: data.to_vec(),
        }
    }

    fn from_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < size_of::<PacketHeader>() {
            return None;
        }

        let header = PacketHeader {
            size: PacketSize::from_be_bytes(bytes[..size_of::<PacketSize>()].try_into().unwrap()),
            packet_type: PacketType(bytes[2]),
        };

        if bytes.len() < header.size as usize {
            return None;
        }

        Some(Self {
            header,
            data: bytes[size_of::<PacketHeader>()..].to_vec(),
        })
    }

    fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(self.header.size as usize);
        bytes.extend_from_slice(&self.header.to_be_bytes());
        bytes.extend_from_slice(&self.data);
        bytes
    }
}


fn main() {
    let packet = Packet::new(&(0..16).collect::<Vec<u8>>());

    let bytes = packet.as_bytes();
    println!("{:?}", bytes);
    // [0, 18, 0, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]
    // u16, type, data
    // big-endian

    let start = std::time::Instant::now();
    for _ in 0..1_000_000 {
        packet.as_bytes();
    }
    println!("{:?}", start.elapsed());  // 250 ms

    let start = std::time::Instant::now();
    for _ in 0..1_000_000 {
        Packet::from_bytes(&bytes).unwrap();
    }
    println!("{:?}", start.elapsed());  // 1.5 s
}