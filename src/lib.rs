use futures::future::join_all;

#[tokio::main]
pub async fn listener_accept_test() {
    let h1 = tokio::spawn(async {
        let start = tokio::time::Instant::now();

        for _ in 0..1000000 {
            let _result = random_calc(5, 2);
        }

        println!("Time elapsed: {:?}", start.elapsed());
    });

    let h2 = tokio::spawn(async {
        let start = tokio::time::Instant::now();

        for _ in 0..1000000 {
            let _result = random_calc(5, 2);
        }

        println!("Time elapsed: {:?}", start.elapsed());
    });
    
    join_all([h1, h2]).await;
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
