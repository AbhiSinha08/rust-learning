use std::ops::Deref;
use std::fmt::Display;

enum List {
    Cons(i32, Box<List>),
    Nil
}

use List::{Cons, Nil};

fn main() {
    let _cons_list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    let x = MyBox::new(5);
    let y = MyBox::new(4);
    let z = MyBox::new(1);
    assert_eq!(5, *x);

    drop(z);
    println!("Main fn ends here.");
}

struct MyBox<T: Display> (T);

impl<T: Display> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T: Display> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
 }

 impl<T: Display> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("Dropping smart pointer with data: {}", self.0)
    }
 }