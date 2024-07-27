#[tokio::main]
pub async fn arc_read(num_clients: u32) {
    use std::sync::{Arc, mpsc};

    let data = Arc::new(String::from("Hello, World!"));
    let (tx, rx) = mpsc::channel();

    for _ in 0..num_clients {
        let data = data.clone();
        let tx = tx.clone();
        tokio::spawn(async move {
            loop {
                let _ = data.len();
                tx.send("Hello").unwrap();
            }
        });
    }

    {
        let timer = std::time::Instant::now();
        let mut cnt = 0;
        for _ in rx {
            cnt += 1;
            if cnt % 1000000 == 0 {
                println!("cnt: {}", cnt);
                println!("time: {:?}", timer.elapsed());
                break;
            }
        }
    }
}


#[tokio::main]
pub async fn static_read(num_clients: u32) {
    use std::sync::mpsc;

    static mut DATA: &str = "Hello, World!";
    let (tx, rx) = mpsc::channel();

    for _ in 0..num_clients {
        let tx = tx.clone();
        tokio::spawn(async move {
            loop {
                unsafe {
                    let _ = DATA.len();
                    tx.send("Hello").unwrap();
                }
            }
        });
    }

    {
        let timer = std::time::Instant::now();
        let mut cnt = 0;
        for _ in rx {
            cnt += 1;
            if cnt % 1000000 == 0 {
                println!("cnt: {}", cnt);
                println!("time: {:?}", timer.elapsed());
                break;
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arc_test() {
        arc_read(10);
    }

    #[test]
    fn static_test() {
        // static_test(10);
    }
}
