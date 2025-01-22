#[macro_export]
macro_rules! make_vec {
    ($($x:expr), *) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
    
    ($x:expr; $y:expr) => {
        {
            let mut temp_vec = Vec::new();
            for _ in 0..$y {
                temp_vec.push($x);
            }
            temp_vec
        }
    };
}