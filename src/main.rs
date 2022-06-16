use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    // prints a line with the message and enters a new line with \n.
    println!("Guess the number!");

    // randomly generated number using the "rand" crate.
    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Please input your guess.");

        // creates a mutable (mut) variable. That receives a new growable string type.
        let mut guess = String::new();

        // uses the standard library to ask for user input.
        io::stdin()
            // reads the input and attributes the value to the 'guess' variable.
            // '&' creates a reference to the guess variable.
            // 'mut' makes that reference mutable.
            .read_line(&mut guess)
            // read_line return a 'Result' type, that return either 'Ok' or 'Err'.
            // 'Ok' indicates success and has the returned value.
            // 'Err' contains an error.
            // 'expect' get's called when there's an error, otherwise the error will break the application.
            .expect("Failed to read line");

        // here we 'shadow' the guess variable and transforms it into a string.
        // the parse method also returns a 'Result' type, that in this case
        // is treated with a match pattern.
        let guess: u32 = match guess.trim().parse() {
            // if 'Ok' just return the value.
            Ok(num) => num,
            // if there's an error just continue.
            Err(_) => continue,
        };

        // prints the number tha the user inputted.
        println!("You guessed: {}", guess);

        // uses the 'cmp' standard library and uses a match to handle each result differently.
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
