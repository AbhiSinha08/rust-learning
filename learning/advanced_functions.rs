fn add_one(x: i32) -> i32 {
    x + 1
}

// Takes only function Pointers
fn do_twice1(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

// Takes either fuction pointer or closure
fn do_twice2<T>(f: T, arg: i32) -> i32
    where T: Fn(i32) -> i32
{
    f(arg) + f(arg)
}
// Fn, FnMut, FnOnce

fn main() {
    let answer = do_twice1(add_one, 5);
    let answer2 = do_twice2(|x| {
        x + 1
    }, 5);

    println!("The answer is: {}", answer);
    println!("The answer is: {}", answer2);


    enum Status {
        Value(u32),
        Stop,
    }

    // Using tuple struct initializer as function pointer to map()
    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
}

// Returning Closures
fn returns_closure() -> impl Fn(i32) -> i32 {
    |x| {x + 1}
}

fn returns_different_closures(b: bool, a: i32) -> Box<dyn Fn(i32) -> i32> {
    if b {
        Box::new(move |x| x + a)
    }
    else {
        Box::new(move |x| x - a)
    }
}