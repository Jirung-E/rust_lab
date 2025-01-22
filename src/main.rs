use rust_lab::make_vec;


fn main() {
    let v = make_vec![1, 2, 3, 4, 5];
    println!("{:?}", v);

    let v = make_vec![1; 10];
    println!("{:?}", v);
}