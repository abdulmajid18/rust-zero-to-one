use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    loop {
        println!("Please in put your guess. !");
        
        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess: u32 = guess.trim().parse().expect("Please input a number!");

        println!("Your guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("You guessed correctly!");
                break;
            }
            Ordering::Less => println!("Too low! Try again."),
            Ordering::Greater => println!("Too high! Try again."),
        }
    }

}
