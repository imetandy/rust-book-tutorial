use std::io;

fn main() {
    println!("Guess the Number!");

    println!("Please input your guess:");

    let mut guess = String::new(); // guess

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    println!("you guessed: {guess}");
}
