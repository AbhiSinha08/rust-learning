mod garden;
use crate::garden::in_garden;
// use modules_crates::add_one;

mod lib;
use crate::lib::add_one;


mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }
}

fn main() {
    in_garden();
    println!("5 + 1: {}", add_one(5));

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // meal.seasonal_fruit = String::from("blueberries");

    use back_of_house::Appetizer;
    let order1 = Appetizer::Soup;
    let order2 = Appetizer::Salad;


    use std::{cmp::Ordering, io};
    use std::io::{self, Write};
}