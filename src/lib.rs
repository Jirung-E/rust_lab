#[tokio::main]
pub async fn mpsc_server(num_clients: u32) {
    use std::sync::mpsc;

    let (sender, receiver) = mpsc::channel::<String>();

    for i in 0..num_clients {
        let sender = sender.clone();
        tokio::spawn(async move {
            loop {
                sender.send(format!("Hello from client {}", i)).unwrap();
            }
        });
    }

    {
        let timer = std::time::Instant::now();

        let mut count = 0;
        for _ in receiver {
            count += 1;
            if count % 10000000 == 0 {
                let elapsed = timer.elapsed();
                println!("mpsc - Received {} messages in {:?}", count, elapsed);
                // break;
            }
        }
    }
}

#[tokio::main]
pub async fn tokio_mpsc_server(num_clients: u32) {
    use tokio::sync::mpsc;

    let (sender, mut receiver) = mpsc::channel(128);

    for i in 0..num_clients {
        let sender = sender.clone();
        tokio::spawn(async move {
            loop {
                sender.send(format!("Hello from client {}", i)).await.unwrap();
            }
        });
    }

    {
        let timer = std::time::Instant::now();

        let mut count = 0;
        loop {
            if let Some(_) = receiver.recv().await {
                count += 1;
                if count % 10000000 == 0 {
                    let elapsed = timer.elapsed();
                    println!("tokio_mpsc - Received {} messages in {:?}", count, elapsed);
                    // break;
                }
            }
        }
    }
}

#[tokio::main]
pub async fn lfqueue_server(num_clients: u32) {
    // use lockfree::queue::Queue;
    use crossbeam::queue::SegQueue as Queue;
    use std::sync::Arc;

    let queue: Arc<Queue<String>> = Arc::new(Queue::new());

    for i in 0..num_clients {
        let queue = queue.clone();
        tokio::spawn(async move {
            loop {
                queue.push(format!("Hello from client {}", i));
            }
        });
    }

    {
        let timer = std::time::Instant::now();

        let mut count = 0;
        loop {
            if let Some(_) = queue.pop() {
                count += 1;
                if count % 10000000 == 0 {
                    let elapsed = timer.elapsed();
                    println!("lfqueue - Received {} messages in {:?}", count, elapsed);
                    // break;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mpsc_test() {
        mpsc_server(10);
    }

    #[test]
    fn lfqueue_test() {
        lfqueue_server(10);
    }
}
