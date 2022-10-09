use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil
}

use List::{Cons, Nil};

fn main() {
    let a = Rc::new(Cons(1, Rc::new(Cons(2, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after dropping c = {}", Rc::strong_count(&a));
}