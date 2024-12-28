use futures::future::join_all;


async fn join_test() {
    let start = std::time::Instant::now();

    let handles = (0..100).map(|n| {
        let n = n + 1;
        tokio::spawn(async move {
            for _ in 0..1000000 {
                let _result = random_calc(5, 2);
            }

            println!("n: {} \t thread: {:?}", n, std::thread::current().id());
        })
    });
    
    join_all(handles).await;
    // for handle in handles {
    //     handle.await.unwrap();
    // }

    println!("Time elapsed: {:?}", start.elapsed());
}

pub fn listener_accept_test() {
    tokio::runtime::Builder::new_multi_thread()
        .worker_threads(1000)
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            join_test().await;
        });
}


fn random_calc(a: i32, b: i32) -> i32 {
    use rand::seq::SliceRandom;
    use rand::thread_rng;

    let op = ["+", "-", "*", "/"];
    let op = *op.choose(&mut thread_rng()).unwrap();

    match op {
        "+" => a + b,
        "-" => a - b,
        "*" => a * b,
        "/" => a / b,
        _ => panic!("Invalid operator"),
    }
}
