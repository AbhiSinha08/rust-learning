fn main() {
    type Kilometers = i32; // Type Alias

    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);
}

fn foo() -> ! {
    // ! = Never type
    // continue, panic!(), loop{} have the ! return type
    panic!()
}

fn generic<T>(t: T) {
    // --snip--
}

fn generic_with_DST<T: ?Sized>(t: &T) {
    // --snip--
}