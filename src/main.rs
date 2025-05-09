use std::io::{self, Write};
use rand::Rng;

fn main() {
    let secret_number = rand::rng().random_range(1..=100);

    println!("Guess the number! It's between 1 and 100.");

    loop {
        print!("Enter your guess: ");
        io::stdout().flush().unwrap();

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };

        if guess < secret_number {
            println!("Higher!");
        } else if guess > secret_number {
            println!("Lower!");
        } else {
            println!("Congrats! You guessed the correct number: {}", secret_number);
            break;
        }
    }
}
