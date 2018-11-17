extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101); // 1-100

    println!("The secret number is {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
                .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num, // return num given by Ok value
            Err(_) => { // Catch all with '_'
                println!("That's not a number! Try again!"); 
                continue; // Continue loop
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too low!"),
            Ordering::Greater => println!("Too high!"),
            Ordering::Equal => {
                println!("You win!");
                break; // Exit loop
            },
        }
    }
}
