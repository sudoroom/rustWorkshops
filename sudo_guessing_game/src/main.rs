use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main() {
    println!("Welcome to the SudoRoom guessing game!");
    let mut guess = String::new();
    
    let secret_number = rand::thread_rng().gen_range(1..=100);
    let secret_two = rand::thread_rng().gen_range(42..=4000);
    println!("The secret number is: {secret_number}");
    println!("The second secret number is {secret_two}");

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");

    // some shadowing
    let guess: u32 = guess
                        .trim()
                        .parse()
                        .expect("Please type a number!");

    match guess.cmp(&secret_number) {
        // a match is made up of arms
        // arm consist of a pattern to match against the code that should be run
        Ordering::Less => println!("Too Small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
