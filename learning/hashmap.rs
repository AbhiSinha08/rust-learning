use std::collections::HashMap;

fn main() {
    let mut count: HashMap<String, u32> = HashMap::new();

    count.insert(String::from("word"), 1);
    count.insert(String::from("word"), 2);

    match count.get("word") {
        Some(num) => println!("The count is: {}", num),
        None => println!("key not present")
    };

    println!("{:?}", count);

    let mut scores = HashMap::new();
    scores.entry(String::from("Blue")).or_insert(10);
    scores.entry(String::from("Red")).or_insert(20);

    let score: i32;

    score = match scores.get("Blue") {
        Some(num) => *num,
        None => 0
    };

    println!("{}", score);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
    
}