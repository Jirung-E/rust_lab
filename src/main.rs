use rust_lab::Player;


fn main() {
    let player = Player::new(3, gmm::Float3::ZERO, gmm::Float4::W, 2, 7.28);
    
    let start = std::time::Instant::now();
    for _ in 0..10000000 {
        let _bytes = player.as_bytes();
    }
    println!("as_bytes: {:?}", start.elapsed());

    let bytes = player.as_bytes();

    let start = std::time::Instant::now();
    for _ in 0..10000000 {
        let _player_copy = Player::from_bytes(&bytes);
    }
    println!("from_bytes: {:?}", start.elapsed());
}


// bytemuck: 900ms, 150ms
// be_bytes: 3.0s, 2.2s
// bincode option건드려서? -> 6.8s, 6.8s
// bytemuck 으로 변환 하고 나서 reverse? 5.2s