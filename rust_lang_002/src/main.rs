use std::io::Write;
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::rng().random_range(1..=10);

    println!("The secret number is: {secret_number}");

    loop {
        print!("Please input your guess: ");
        io::stdout().flush().unwrap();

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = guess.trim().parse().expect("Please type a number!");

        print!("You guessed: {guess}");
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("\nToo small!"),
            Ordering::Greater => println!("\nToo big!"),
            Ordering::Equal => {
                println!("\nYou win!");
                break;
            },
        }
        println!("Guess again!");
    }
}