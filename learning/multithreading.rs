use std::thread;
use std::time::Duration;
use std::sync::mpsc;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..7 {
            println!("no. {} from spawed thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..4 {
        println!("no. {} from main thread", i);
        thread::sleep(Duration::from_millis(1));
    }
    handle.join().unwrap();

    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("The vector is {:?}", v);
    });
    handle.join().unwrap();


    let (tx, rx) = mpsc::channel();
    let tx2 = tx.clone();

    thread::spawn(move || {
        for msg in 1..5 {
            tx.send(msg);
            thread::sleep(Duration::from_millis(500));
        }
    });
    thread::spawn(move || {
        for msg in 11..15 {
            tx2.send(msg);
            thread::sleep(Duration::from_millis(500));
        }
    });

    let msg = rx.recv().unwrap();
    println!("Got: {}", msg);

    for msg in rx {
        println!("Got: {}", msg);
    }
}