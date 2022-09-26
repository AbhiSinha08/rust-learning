use std::convert::TryInto;

fn main() {
    let x: u8 = u8::MAX;
    println!("x: {x}");
    let x = (0, 1);
    println!("{} {}", x.0, x.1);

    let y: (i32, bool, f64) = (5, true, 6.4);
    let (a, b, c) = y;
    println!("{a} {b} {c}");

    // Comment
    /* Comment */

    let a;
    a = 6;

    // let mut b;
    // b = 6; b = 5;

    println!("{a}, {b}");

    let arr = [4, 5, 6];
    println!("{} length: {},", arr[0], arr.len());

    let arr: [u32; 5] = [1; 5];
    println!("arr[1]: {}, length: {}", arr[1], arr.len());
    println!("\n");


    let x: i32 = 5;
    println!("return: {}", add_one(x));

    let x: u32 = 5;
    println!("return: {}", add_one(x.try_into().unwrap()));

    println!("return: {}", add_one(5));

    let x: i64 = 5;
    println!("return: {}", add_one(x.try_into().unwrap()));

    let (a, b) = give_tuple(10, 11);
    println!("a: {a}, b: {b}");
    println!("\n");


    let x = if false {
        5
    }
    else {
        6
    };
    println!("{x}");

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");

    'loop_label_1: while 0 < 1 {
        for i in 0..5 {
            println!("i: {i}");
            if i == 3 {
                break 'loop_label_1;
            }
        }
    }

    for num in (1..=4).rev() {
        print!("num: {num}, ");
    }
}

fn add_one(x: i32) -> i32 {
    x + 1
}

fn give_tuple(x: i32, y: i32) -> (i32, i32) {
    println!("Returning tuple...");
    (x, y)
    // loop {
    //     break (x, y);
    // }
}