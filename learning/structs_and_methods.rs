use std::convert::TryInto;

#[derive(Debug)]
struct Circle {
    radius: u32,
    center: (i32, i32)
}

impl Circle {
    fn new(x: i32, y: i32, radius: u32) -> Circle {
        Circle {
            radius,
            center: (x, y)
        }
    }

    fn area(&self) -> f32 {
        let r_sq = self.radius * self.radius;
        3.14 * r_sq as f32
    }

    fn is_overlapping(&self, other: &Circle) -> bool {
        let x_diff = self.center.0 - other.center.0;
        let y_diff = self.center.1 - other.center.1;

        let distance_sq = (x_diff * x_diff) + (y_diff * y_diff);
        let distance_sq: u32 = distance_sq.try_into().unwrap();

        let r_sum = self.radius + other.radius;
        let r_sum_sq = r_sum * r_sum;

        distance_sq < r_sum_sq
    }

    fn expand(&mut self) {
        self.radius += 1;
    }
}

fn main() {
    let mut c1 = Circle::new(1, 1, 4);
    let c2 = Circle::new(4, 3, 2);

    dbg!(&c1);
    dbg!(&c2);

    let a1 = c1.area();
    let a2 = c2.area();

    println!("area of c1: {}, area of c2: {}", a1, a2);

    match c1.is_overlapping(&c2) {
        true => println!("The circles are overlapping"),
        false => println!("The circles are not overlapping")
    };

    c1.expand();
    dbg!(&c1);
}