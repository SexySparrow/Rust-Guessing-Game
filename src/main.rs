use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    println!("Guess the number between 0 and 100!");

    let mut rng = rand::thread_rng();
    let unknown: u32 = rng.gen_range(0..100);

    loop {
        println!("Please input your guess.");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                print!("Please write a number!");
                continue;
            }
        };

        println!("You guessed: {guess}");
        match guess.cmp(&unknown) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
