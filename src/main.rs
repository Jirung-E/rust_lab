fn main() {
    let s1 = b"hello ";
    let s2 = b"world";
    println!("{:?}", s1);
    println!("{:?}", s2);

    let concat = [s1.to_vec(), s2.to_vec()].concat();
    println!("{:?}", concat);

    let mut v1 = s1.to_vec();
    v1.extend_from_slice(s2);
    println!("{:?}", v1);

    let v2: Vec<_> = s1.iter().chain(s2.iter()).collect();
    println!("{:?}", v2);
}