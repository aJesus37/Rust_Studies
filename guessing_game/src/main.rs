extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    //Creates a random number between 1 an 100
    let secret_number = rand::thread_rng().gen_range(1, 101);
    //Loops through the core of the program
    loop {
        println!("Please input your guess.");
        //Create a new MUTABLE variable of type string
        let mut guess = String::new();
        // Ask for user input and print the error message if goes wrong
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
        //Parse input to be a num or reiterate the loop
        //Also handles errors in the parsing with the match arms
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);
        // Compare the input value with the secret number, outputing if it
        // is greater, less or equal the value.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
    
}