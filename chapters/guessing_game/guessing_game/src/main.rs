use std::io;

fn main() {
    println!("guess the  number!");
    let target = 42;
    println!("please input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
                .expect("Failed to read line");

    println!("you guessed: {}", guess);

    match guess.trim().parse::<i32>() {
        Ok(number) => {
            if number == target {
                println!("The input matches {}", number);
            } else {
                println!("You didn't get it");
            }
        },
        Err(_) => println!("That's not the valid number"),
    }

}
