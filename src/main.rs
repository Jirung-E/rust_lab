// use rust_lab::mpsc_server;
use rust_lab::tokio_mpsc_server;
// use rust_lab::lfqueue_server;

fn main() {
    let num_clients = 10;

    // mpsc_server(num_clients);
    tokio_mpsc_server(num_clients);
    // lfqueue_server(num_clients);
    
    // {
        // use std::thread;
        // let mpsc_handle = thread::spawn(move || {
        //     mpsc_server(num_clients);
        // });
        // let lfqueue_handle = thread::spawn(move || {
        //     lfqueue_server(num_clients);
        // });

        // mpsc_handle.join().unwrap();
        // lfqueue_handle.join().unwrap();
    // }
}