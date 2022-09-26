fn main() {
    let x: i32 = 5;
    let y = x;
    println!("x: {}, y: {}", x, y);

    let x: &str = "Hello";
    let y = x;
    println!("x: {}, y: {}", x, y);

    let s1: String = String::from("Hello World");
    // let s2 = s1;
    let s2 = s1.clone();
    println!("s1: {}, s2: {}", s1, s2);

    let s2: &str = &s1;
    println!("s1: {}, s2: {}", s1, s2);

    println!("");

    let mut s1 = String::from("String");
    let s2 = &mut s1;
    // let s3 = &mut s1;
    println!("s2: {}", s2);

    let s2 = &s1;
    let s3 = &s2;
    println!("s1: {}, s2: {}", s2, s3);

    read_str(&s1);
    read_str(s2);

    modify(&mut s1);
    println!("s1: {}", s1);
    // println!("s2: {}", s2);
}

fn read_str(s: &str) {
    println!("s: {}", s);
}

fn modify(s: &mut String) {
    s.push_str(" gg");
}