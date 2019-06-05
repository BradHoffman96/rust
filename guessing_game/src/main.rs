extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess that number!!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess: ");

        //'let' is used to create variables (immutable by default)
        //'mut' allows a variable to be mutable
        //'String::new()' returns a new instance of String
        //'::new' implies that new is an associated function of the String type
        //  Associated functions are implemented on types, rather than on instances of Types
        let mut guess = String::new();

        //'&' is used as a reference (immutable by default)
        //Result types are enums (a type that can have a fixed set of values)
        //  Those values are called the enum's 'variants'
        //  Result variants are 'Ok' and 'Err'
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line.");

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
            },
        }
    }
}
