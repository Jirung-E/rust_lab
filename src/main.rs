use std::collections::VecDeque;
use std::ptr::addr_of;


fn main() {
    let mut queue = VecDeque::from([
        1, 2, 3, 4, 5,
        6, 7, 8, 9, 10, 
    ]);

    queue.iter().for_each(|x| {
        println!("value: {} \t addr: {:?}", x, addr_of!(*x));
    });

    queue.pop_front();
    queue.pop_front();
    queue.pop_front();
    queue.push_front(11);
    queue.push_front(11);
    queue.push_front(11);
    queue.push_front(11);
    queue.push_front(11);
    queue.push_front(11);
    queue.push_front(11);
    queue.push_front(11);
    queue.push_front(11);
    queue.push_front(11);
    queue.push_front(11);
    queue.push_front(11);

    queue.iter().for_each(|x| {
        println!("value: {} \t addr: {:?}", x, addr_of!(*x));
    });
}