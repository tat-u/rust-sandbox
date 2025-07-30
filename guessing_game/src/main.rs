use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let mut randgen = rand::thread_rng(); // Initialize random number generator
    let range = 1..101; // Range literal. Range is half-open by default in Rust

    let secret_number = randgen.gen_range(range);

    // In Rust, infinite loop be like this
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        // &mut guess is "mutable reference"
        // It IS different from *mut guess (raw pointer), which bypasses Rust's safety checks
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line"); // .expect means "panic if it returns Err"

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,     // return num if Ok
            Err(_) => continue, // continue to the next iteration if Err
        };

        println!("You guessed: {}", guess);

        // &secret_number is just a "reference" to secret_number
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
