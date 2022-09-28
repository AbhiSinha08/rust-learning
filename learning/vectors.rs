enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let mut v1: Vec<i32> = Vec::new();

    v1.push(1);
    v1.push(2);
    v1.push(3);

    for i in &v1 {
        println!("{}", i);
    }

    println!("{}", &v1[0]);

    match v1.get(20) {
        Some(i) => {
            println!("{}", i);
        }
        None => (),
    }

    let mut v2 = vec![1, 2, 3];
    for i in &mut v2 {
        *i = *i + 10;
    }

    use SpreadsheetCell as Cell;
    let row = vec![
        Cell::Int(3),
        Cell::Text(String::from("blue")),
        Cell::Float(10.12),
    ];
}
