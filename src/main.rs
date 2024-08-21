#[derive(serde::Serialize, serde::Deserialize)]
struct Packet {
    size: u16,
    data: Vec<u8>,
}


fn main() {
    let packet = Packet {
        size: 16,
        data: (0..16).collect(),
    };

    println!("{:?}", bincode::serialize(&packet).unwrap());
    // [16, 0, 16, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]
    // u16, u64(usize), data
    // little endian
}