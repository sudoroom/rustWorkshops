extern crate rand;
use std::io;
use rand::Rng;
use std::cmp::Ordering;
use rand::distributions::{Uniform};

fn main() {
    println!("guess the  number!");
    let target = 42;

    let secret_number = rand::thread_rng().sample(Uniform::new(1, 101));
    println!("The secret number is {}", secret_number);
    println!("please input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
                .expect("Failed to read line");

    println!("you guessed: {}", guess);

    match guess.trim().parse::<i32>() {
        Ok(number) => {
            if number == target {
                println!("The input matches the target which is{}", number);
            } else {
                println!("You didn't get the target");
            }
        },
        Err(_) => println!("That's not a valid number"),
    }

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("you win!");
    }

    match guess.trim().parse::<i32>() {
        Ok(number) => {
            if number == secret_number {
                println!("Hey you got the secret number!");
            } else {
                println!("you didn't get the secret number this time");
            }
        },
        Err(_) => println!("That's not a valid input of a number"),
    }
}
