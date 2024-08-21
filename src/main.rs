struct Packet {
    size: u16,
    data: Vec<u8>,
}

impl Packet {
    fn new(data: &[u8]) -> Self {
        Self {
            size: data.len() as u16 + 2,
            data: data.to_vec(),
        }
    }

    fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = bincode::serialize(&self.size).unwrap();
        bytes.extend_from_slice(&self.data);
        bytes
    }
}


fn main() {
    let packet = Packet::new(&(0..16).collect::<Vec<u8>>());

    println!("{:?}", packet.as_bytes());
    // [18, 0, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]
    // u16, data
    // little endian
}