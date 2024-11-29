//Guessing Game//
//Libraries Used//
use std::io;
use rand::Rng;
use std::cmp::Ordering;
use colored::*;
//Functions
fn main () {
//calles a new secret number variable using the rand library
//generates the secret_number using an INCLUSIVE range from 1 - 100
    let secret_number = rand::thread_rng().gen_range(1..=100);

//Prompts user for input
//starts gameplay loop
//calls new string var (guess)
//writes user input to guess with a reference as to not take ownership
    println!("{}", "Hello user, please enter a number~".blue());

    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Cant read line");
        
//Takes the user guess, trims whitespace, parses it into u32
//if its a valid u32 int, it continues with the rest of the game
//if it is anything else it forces the user to reinput a number
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{}", "Input a number".red());
                continue;
            }
        };


        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too Small".red()),
            Ordering::Greater => println!("{}", " Too Big".red()),
            Ordering::Equal => {
                println!("{}", " Perfect!".green());
                break;
            },
        }
    }
}
