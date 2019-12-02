use std::io;
use std:cmp::Ordering;
use rand::Rng;

fn main() {

    println!("Try and guess the number!");

    let sec_num = rand::thread_rng().gen_range(1, 101);

    println!("Please type a guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse()
        .expect("numbers only plz");

    println!("you guessed {}.", guess);

    println!("the secret number is {}", sec_num);

    match.guess.cmp(&sec_num) {
        Ordering::Less => println!("Oi that number is too weee"),
        Ordering::Equal => println!("Shit you got it my guy"),
        Ordering::More => println!("Thats 2 big 4 irl daddy-o"),
    }
}
