#[derive(Debug)]
enum Shape {
    None,
    Rectangle(u32, u32),
    Circle { center: (i32, i32), radius: u32 }
}

fn main() {
    let mut rect = Shape::Rectangle(4, 3);

    rect = Shape::None;

    match rect {
        Shape::Rectangle(x, y) => println!("{} {}", x, y),
        _ => println!("Not a rectangle")
    }


    let a = Some(4);
    let b = Some(6);
    let c: Option<i32> = None;

    let d = add(a, b);
    let e = add(a, c);

    println!("d: {}, e: {}", d, e);
}

fn add(a: Option<i32>, b: Option<i32>) -> i32 {
    let mut sum: i32 = match a {
        Some(num) => num,
        None => 0
    };
    sum += match b {
        Some(num) => num,
        None => 0
    };

    sum
}