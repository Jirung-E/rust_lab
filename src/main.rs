fn main() {
    let v = (0..10).map(|x| 
            (0..10).map(move |y| (x, y)).collect::<Vec<_>>()
        ).collect::<Vec<_>>();
    println!("{:?}", v);

    let folded = v.iter().enumerate().fold(Vec::new(), |mut acc, x| {
        if x.0 % 2 == 0 {
            acc.extend(x.1.iter().cloned());
        }
        acc
    });
    println!("{:?}", folded);
}