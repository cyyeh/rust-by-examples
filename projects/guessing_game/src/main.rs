// The Rust Prelude: https://doc.rust-lang.org/std/prelude/index.html
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    // println is a macro that prints a string to the screen
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Please input your guess.");

        // 1. we use the let statement to create the variable
        // 2. variables are immutable by default
        // 3. add mut before the variable name to make a variable mutable
        // 4. String::new returns a new instance of a String, which is growable, UTF-8 encoded bit of text
        // 5. ::new means new is the method of the String type
        let mut guess = String::new();

        // 1. The full job of read_line is to take whatever the user types into standard input and
        // append that into a string (without overwriting its contents)
        // 2. The & indicates that this argument is a reference,
        // which gives you a way to let multiple parts of your code 
        // access one piece of data without needing to copy that data into memory multiple times.
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

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