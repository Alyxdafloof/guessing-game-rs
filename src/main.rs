//Guessing Game//
//Libraries Used//
use std::io;
//Functions
fn main () {
    println!("Hello user, please enter a number~");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Cant read line");
    let guess: u32 = guess.trim().parse().expect("couldnt trim and parse");

    println!("user guess {}", guess);
}
