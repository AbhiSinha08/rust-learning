fn greatest<T: PartialOrd + Copy>(v: &Vec<T>) -> T {
    let mut x = v.get(0).unwrap();
    
    for num in v {
        if num > x {
            x = num;
        }
    }

    *x
}

fn main() {
    let v = vec![2, 3, 4, 7, 1];
    let num = greatest(&v);
    println!("The greatest num in vector is {}", num);

    let v = vec!['a', 'b', 'd', 'x', 'x', 'e'];
    let char = greatest(&v);
    println!("The greatest char in vector is {}", char);
    println!("The vector is {:?}", v);


    let p1: Point<u16> = Point {
        x: 3,
        y: 4
    };

    let p2: Point<f32> = Point {
        x: 5.0,
        y: 6.0
    };

    println!("{}", p1.x());
    // println!("{}", p1.y());
    println!("{}", p2.x());
    println!("{}", p2.y());

}

struct Point<T> {
    x: T,
    y: T
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn y(&self) -> &f32 {
        &self.y
    }
}