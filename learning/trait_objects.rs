pub mod gui_lib {
    pub trait Drawable {
        fn draw(&self);
    }
    pub struct Screen {
        pub components: Vec<Box<dyn Drawable>>
    }
    impl Screen {
        pub fn run(&self) {
            for component in &self.components {
                component.draw();
            }
        }
    }

    pub struct Button {
        pub size: (u32, u32),
        pub label: String
    }
    impl Drawable for Button {
        fn draw(&self) {
            println!("Drawing the button {}", self.label);
        }
    }
}

// Custom component
struct Label {
    name: String
}
impl Drawable for Label {
    fn draw(&self) {
        println!("Drawing the label {}", self.name);
    }
}

use gui_lib::*;
fn main() {
    let screen = Screen {
        components: vec![
            Box::new(Button {
                size: (100, 120),
                label: String::from("Submit")
            }),
            Box::new(Label {
                name: String::from("Welcome")
            })
        ]
    };
    screen.run();
}