use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let m = Mutex::new(0);
    {
        let mut num = m.lock().unwrap();
        *num = 1;
    }
    let num = m.lock().unwrap();
    println!("The value in mutex is {}", num);
    drop(num);

    let num = m.lock().unwrap();
    println!("Accessing num again after dropping the prev lock: {}", num);
    println!("{:?}", m);


    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        handles.push(
            thread::spawn(move || {
                // let mut num = (*counter).lock().unwrap();
                let mut num = counter.lock().unwrap();
                *num += 1;
            })
        )
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    let num = counter.lock().unwrap();
    println!("The value in counter: {}", num);
}