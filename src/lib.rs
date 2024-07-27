use tokio::net::TcpListener;

#[tokio::main]
pub async fn listener_accept_test() {
    let listener = TcpListener::bind("localhost:8080").await.unwrap();

    let mut counter = 0;
    loop {
        match listener.accept().await {
            Ok(_) => {
                counter += 1;
                if counter & 1 == 0 {
                    tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
                }
                println!("counter: {}", counter);
            }
            Err(e) => {
                println!("Failed to accept connection: {:?}", e);
            }
        }
    }
}
