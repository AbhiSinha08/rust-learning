use std::ops;
use std::fmt;

pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
struct Counter{}
impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        Some(0)
    }
}

struct Point { x: u32, y: u32 }
impl ops::Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

// Default Generic Type Parameter
trait Add<Rhs=Self> {
    type Output;

    fn add(self, rhs: Rhs) -> Self::Output;
}

struct Millimeters(u32);
struct Meters(u32);

// If we want to change generic type
impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}


trait PrettyPrint: fmt::Display {
    fn print(&self) {
        let output = self.to_string(); // fn in fmt::Display trait
        println!("*");
        println!("{}", output);
        println!("*");
    }
}
impl PrettyPrint for Point {}
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn main() { }