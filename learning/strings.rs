fn main() {
    let mut s1 = String::new();

    let s: &str = "hello";
    s1 = s.to_string();
    s1.push(' ');
    s1.push_str("world");
    
    println!("{}\n", s1);


    let s1 = String::from("Hello, ");
    let s2 = String::from("world");
    let s3 = String::from("!");
    let s3 = s1 + &s2 + &s3;

    // println!("{}", s1);
    println!("{}", s2);
    println!("{}", s3);
}