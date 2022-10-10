struct Point {
    x: i32,
    y: i32,
}
enum Message {
    Move{x: i32, y: i32},
    Write(String),
}

fn main() {
    let x = Some(3);
    let y = 3;
    match x {
        Some(1 | 2) => println!("one or two"),
        Some(n) if n == y => println!("y: {}", y),
        Some(n) if n % 2 == 0 => println!("even"),
        Some(y) => println!("x: {}", y),
        None => println!("none")
    };

    let x = 'c';
    match x {
        'a'..='f' => println!("Till f"),
        _ => println!("After f")
    };

    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }

    let m = Message::Move {x: 0, y: 1};
    match m {
        Message::Move {x, y} => println!("X: {}, Y: {}", x, y),
        Message::Write(s) => println!("{}", s)
    };

    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, .., last) => {
            println!("Edge numbers: {first}, {last}");
        }
    };

    let m = Message::Move {x: 0, y: 1};
    match m {
        Message::Move {x: x_var @ 0..=1, y} => println!("x in range: {}, {}", x_var, y),
        _ => ()
    };
}

fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {}", y);
}