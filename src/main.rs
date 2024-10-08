use std::collections::VecDeque;
use std::ptr::addr_of;


fn main() {
    let queue = VecDeque::from([
        1, 2, 3, 4, 5,
        6, 7, 8, 9, 10, 
        11, 12, 13, 14, 15,
        16, 17, 18, 19, 20
    ]);

    queue.iter().for_each(|x| {
        println!("value: {} \t addr: {:?}", x, addr_of!(*x));
    });
}