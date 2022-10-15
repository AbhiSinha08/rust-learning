mod library {
    #[macro_export] // makes macro available when mod is in scope
    macro_rules! vec {
        ( $( $x:expr ),* ) => {
            {
                let mut temp_vec = Vec::new();
                $(
                    temp_vec.push($x);
                )*
                temp_vec
            }
        };
    }
}

fn main() {
    let v = vec![3, 6];   
    println!("{:?}", v);

    // The generated code will be:
    {
        let mut temp_vec = Vec::new();
        temp_vec.push(3);
        temp_vec.push(6);
        temp_vec
    };
}