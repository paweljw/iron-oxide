extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

const MIN_NUMBER: u32 = 1;
const MAX_NUMBER: u32 = 21;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(MIN_NUMBER, MAX_NUMBER);

    loop {
        println!("Input your guess:");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input a number :/");
                continue;
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less    => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal   => {
                println!("You win!");
                break;
            }
        };
    }
}
