use std::io;

fn main() {
    println!("Enter a number: ");
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess);

    println!("You guessed: {}", guess);
}

https://doc.rust-lang.org/stable/book/ch02-00-guessing-game-tutorial.html?highlight=prelude#processing-a-guess
Generating a Secret Number
