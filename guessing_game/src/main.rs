use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Secret number is {secret_number}");

    println!("Please input your guess!");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Failed to parse the integer");

    match guess.cmp(&secret_number) {
        Ordering::Equal => println!("You won!"),
        Ordering::Greater => println!("You guessed higher, better luck next time!"),
        Ordering::Less => println!("You guessed lower, better luck next time!"),
    }

    println!("You guessed {guess}");
}
