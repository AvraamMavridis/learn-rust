extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {

    let _secret = rand::thread_rng().gen_range(1,101);


    loop {
        let mut guess = String::new();
        println!("What is your guess?");

        io::stdin().read_line(&mut guess)
            .expect("Failed to read user input");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, 
        };

        match guess.cmp(&_secret) {
            Ordering::Equal => {
                println!("You win!");
                break;
            },
            Ordering::Greater => println!("Too big!"),
            Ordering::Less => println!("Too small!")
        }
    }


    
    println!("Secret {}", _secret);
}
