use rand::Rng;
use std::cmp::Ordering::*;
use std::io::{self, Write};

fn main() {
    println!("Welcome to the Guessing game");

    let number = rand::thread_rng().gen_range(1..101);

    // println!("{number}");

    loop {
        print!("Guess a number: ");
        io::stdout().flush().unwrap();

        let mut guessed = String::new();

        io::stdin()
            .read_line(&mut guessed)
            .expect("Cannot read from the stdin");

        let guessed_num: i32 = match guessed.trim().parse() {
            Ok(num) => num,
            Err(_e) => {
                println!("Please guess a number\n");
                continue;
            }
        };

        println!("You guessed {guessed_num}");

        match guessed_num.cmp(&number) {
            Less => println!("The guesssed number is too less.\n"),
            Equal => {
                print!("Well Done! ");
                println!("You guessed the number correctly");
                break;
            },
            Greater => println!("The guessed number is too high.\n")
        }
    }
}
