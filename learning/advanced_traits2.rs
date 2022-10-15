trait Pilot { fn fly(&self); }
trait Wizard { fn fly(&self); }
struct Human {}

impl Pilot for Human {
    fn fly(&self) {
        println!("Pilot Method");
    }
}
impl Wizard for Human {
    fn fly(&self) {
        println!("Wizard Method");
    }
}
impl Human {
    fn fly(&self) {
        println!("Own Implementation");
    }
}

trait Pilot2 {
    fn fly(); //Associated function, not a method
}
struct Human2 {}
impl Pilot2 for Human2 {
    fn fly() {
        println!("Associated funtion of pilot2 trait");
    }
}
impl Human2 {
    fn fly() {
        println!("Own Implementation Associated Function");
    }
}

fn main() {
    let human = Human {};
    human.fly();
    Pilot::fly(&human);
    Wizard::fly(&human);

    Human2::fly();
    // Pilot2::fly(); //Error
    // Fully Qualified Syntax
    <Human2 as Pilot2>::fly();
}