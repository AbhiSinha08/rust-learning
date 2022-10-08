fn main() {
    let v1 = vec![1,2,3];
    let v1_iter = v1.iter();
    for item in v1_iter {
        println!("{}", item);
    }

    let mut v1_iter = v1.iter();
    assert_eq!(v1_iter.next(), Some(&1));

    let total: i32 = v1_iter.sum();
    println!("Sum: {}", total); // 5 because 1 is already skipped

    let iter2 = v1.iter().map(|x| x + 1);
    let v2: Vec<_> = iter2.collect();
    assert_eq!(v2, vec![2, 3, 4]);


    let mut counter = Counter::new();
    for _ in 0..5 {
        print!("{}, ", counter.next().unwrap());
    }

    // // Infinite loop
    // let counter = Counter::new();
    // for i in counter {
    //     println!("{}", i);
    // }
}

struct Counter {
    count: u32
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        Some(self.count)
    }
}