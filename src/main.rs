use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    // Generating random number between 1 and 100 included
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // Looping through users input and checking for correct answer, program quits when correct guess is entered
    loop {
        println!("Please input your guess");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read a line!");
        // Taking guess variable and parsing it from string to u32 type, we also trim whitespace and cover result error
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}.");

        // Using Ordering to compare the secret_number and guess variables to find match
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!!!"),
            Ordering::Greater => println!("To big!!!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
