use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_num = rand::thread_rng().gen_range(1..=2);

    println!("Enter a number: ");
    let mut guess = String::new();

    
    while true {
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        match guess.trim().cmp(&secret_num.to_string()) {
            Ordering::Equal => {
                println!("You guessed correctly! secret_num: {secret_num} guess: {guess}");
                return
            }
            Ordering::Less => {
                println!("You guessed incorrectly!: guess:{guess} is less than secret num {secret_num}");
            }
            Ordering::Greater => {
                println!("You guessed incorrectly!: guess:{guess} is greater than secret num{secret_num}");
            }
        }
    }

}

//https://doc.rust-lang.org/stable/book/ch02-00-guessing-game-tutorial.html?highlight=prelude#processing-a-guess
//Comparing the Guess to the Secret Number