use std::{io, env};

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(args);
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}

