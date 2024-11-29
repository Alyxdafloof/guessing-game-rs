//Guessing Game//
//Libraries Used//
use std::io;
use rand::Rng;
use std::cmp::Ordering;
//Functions
fn main () {
//calles a new secret number variable using the rand library
//generates the secret_number using an INCLUSIVE range from 1 - 100
    let secret_number = rand::thread_rng().gen_range(1..=100);

//Prompts user for input
//starts gameplay loop
//calls new string var (guess)
//writes user input to guess with a reference as to not take ownership
//then makes guess a u32 int by triming and parsing it, also now making it immutible
    println!("Hello user, please enter a number~");

    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Cant read line");
        let guess: u32 = guess.trim().parse().expect("couldnt trim and parse");


        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal => {
                println!("Perfect!");
                break;
            },
        }
    }
}
