use std::io::Write;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        print!("Enter your guess: ");

        // how to do that more humble, like "<< std::endl"?
        std::io::stdout().flush().unwrap();

        let mut guess = String::new();

        std::io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
