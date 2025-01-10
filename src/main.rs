use rust_lab::{
    math,
    game::{World, Player},
};
use std::io::Write;
use tokio::sync::broadcast;
use tokio::time::Duration;


const NUM_CLIENTS: usize = 10000;
const TEST_TIME: Duration = Duration::from_secs(10);


async fn handle_client(_id: usize, mut rx: broadcast::Receiver<World>) {
    // for _ in 0..NUM_SEND {
    loop {
        let _world = match rx.try_recv() {
            Ok(world) => {
                // println!("{} - ok", id);
                world
            },

            Err(broadcast::error::TryRecvError::Lagged(_n)) => {
                // println!("{} - 잠깐 자고왔더니 {}개의 메시지를 놓쳤네요!", id, n);
                continue;
            },

            Err(_) => continue,
        };
    }
}


#[tokio::main]
async fn main() {
    let mut world = World::new();
    for i in 0..NUM_CLIENTS {
        let mut player = Player::new(format!("p{}", i).as_str());
        player.position = math::Vector3::random();
        world.add_player(player);
    }

    let (tx, rx) = broadcast::channel(1);

    let _handles = (0..NUM_CLIENTS).map(|i| {
        let rx = rx.resubscribe();
        tokio::spawn(async move {
            handle_client(i, rx).await;
        })
    }).collect::<Vec<_>>();

    let timer = tokio::time::Instant::now();
    let mut prev_elapsed = Duration::from_secs(0);
    let mut send_cnt = 0;
    loop {
        let elapsed = timer.elapsed();
        if (elapsed - prev_elapsed).as_secs() >= 1 {
            print!("\rsend_per_sec: {}", send_cnt);
            std::io::stdout().flush().unwrap();
            send_cnt = 0;
            prev_elapsed = elapsed;
        }
        if elapsed > TEST_TIME {
            break;
        }
        tx.send(world.clone()).unwrap();
        send_cnt += 1;
    }

    println!("\nDone!");
}