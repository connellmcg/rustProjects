use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("The secret number is : {}", secret_number);

    loop {
    println!("Please input your guess:");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line.");

    //Make sure the number is a number and not text
    //The trim eliminates the \r\n from the input
    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };
    

    println!("You guessed: {}", guess);

    //Compare the input to the scret number and break when it matches
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too high!"),
        Ordering::Equal => {
            println!("Well done!");
            break;
        }
    }
    }
}
