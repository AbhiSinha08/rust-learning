use std::time::Duration;
use std::thread;

fn main() {
    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    fn  add_one_v1   (x: u32) -> u32 { x + 1 }
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x|             { x + 1 };
    let add_one_v4 = |x|               x + 1  ;

    add_one_v3(3);
    add_one_v4(3);

    let example_closure = |x| x;

    let s = example_closure(String::from("hello"));
    // let n = example_closure(5);


    let mut list = vec![1, 2, 3];

    let only_borrows = || println!("From closure: {:?}", list);
    let mut borrows_mutably = || list.push(7);
    let takes_ownership = move || println!("From thread: {:?}", list);
}

// Closure traits
// FnOnce FnMut Fn