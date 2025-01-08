fn main() {
    let v = (0..50)
        .flat_map(|x| 
            (0..50).map(move |y| (x, y)).collect::<Vec<_>>()
        ).collect::<Vec<_>>();
    println!("{:?}", v);
}