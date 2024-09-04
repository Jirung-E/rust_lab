use lockfree::stack::Stack;
use lockfree::queue::Queue;
use std::thread;
use std::sync::Arc;

const MAX_PUSHES: usize = 10_000_000;

pub fn run(threads: usize) {
    let stack = Arc::new(Stack::new());
    
    let start = std::time::Instant::now();
    
    let handles = (0..threads).map(|i| {
        let stack = stack.clone();
        thread::spawn(move || {
            for _ in 0..MAX_PUSHES/threads {
                stack.push(i);
            }
        })
    }).collect::<Vec<_>>();

    for handle in handles {
        handle.join().unwrap();
    }

    println!("{} threads: {} ms", threads, start.elapsed().as_millis());

    let mut count = 0;
    while let Some(_) = stack.pop() {
        count += 1;
    }

    println!("  count: {}", count);
}
