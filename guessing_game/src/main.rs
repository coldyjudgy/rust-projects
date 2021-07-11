extern crate rand; // new crate added. alternative: "use rand"

use std::io; // use Input/Output functionality in std(The Rust Standard Library)
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is: {}", secret_number);

    loop{
        println!("Please input your guess.");

        // "let" is for a constant. "mut" stands for mutable.
        let mut guess = String::new(); 

        // std::io::stdin() needed without line 1.
        io::stdin().read_line(&mut guess) // stdin() returns a Stdin instance. // & stands for a reference, which is immutable.
            .expect("Failed to read line"); // read_line() returns a io::Result instance, which has expext method.

        /*  Error: 'main' panicked 
        let guess: u32 = guess.trim().parse()
            .expect("Please type a number!");
        */

        /*  Handling error by falling back to a default value
        let guess: u32 = guess.trim().parse()
            .unwrap_or(29);
        */
        
        let guess: u32 = match guess.trim().parse() { // trim() removes /n.
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
