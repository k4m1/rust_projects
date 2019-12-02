use std::io;

fn main() {

    println!("Try and guess the number!");

    println!("Please type a guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("you guessed {}.", guess);
}
