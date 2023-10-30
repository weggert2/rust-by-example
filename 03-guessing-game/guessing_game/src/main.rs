use rand::Rng;
use std::cmp;
use std::io;

fn main() {
    println!("Guess the number [0, 100].");

    let low_bound = 1;
    let high_bound = 100;
    let answer = rand::thread_rng().gen_range(low_bound..=high_bound);

    let mut low_guess = 0;
    let mut high_guess = 100;
    let mut guess_num = 0;

    // Loop until the user gets the right answer.
    loop {
        let mut guess_str = String::new();
        guess_num += 1;

        io::stdin()
            .read_line(&mut guess_str)
            .expect("Failed to read user input");

        // Parse the guess. If it is not a number, yell at the user and
        // continue to the next iteration of the loop.
        let guess_u32: u32 = match guess_str.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!(
                    "Guess #{guess_num}: '{}' is not a number, Try again!",
                    guess_str.trim()
                );
                continue;
            }
        };

        print!("Guess #{guess_num}: {guess_u32}. ");

        // Compare against the right answer. Keep track of the user's highest
        // and lowest guess, and suggest a good next guess.
        match guess_u32.cmp(&answer) {
            cmp::Ordering::Less => {
                print!("Too low. ");
                low_guess = cmp::max(guess_u32, low_guess);
                print!(
                    "You should guess in between {} and {high_guess}. ",
                    low_guess + 1,
                );

                println!("I suggest {}", (low_guess + high_guess) / 2);
            }
            cmp::Ordering::Greater => {
                print!("Too high. ");
                high_guess = cmp::min(guess_u32, high_guess);
                print!(
                    "You should guess in between {low_guess} and {}. ",
                    high_guess - 1
                );

                println!("I suggest {}", (low_guess + high_guess) / 2);
            }
            cmp::Ordering::Equal => {
                println!("Corrrect! You took {guess_num} tries");
                break;
            }
        }
    }
}
