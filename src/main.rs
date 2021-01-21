use rand::Rng;
use std::io;

fn main() {
    println!("Guess Number");

    println!("Please enter your number: ");

    let secrete_number = rand::thread_rng().gen_range(1, 101);

    let mut number = String::new();

    io::stdin()
        .read_line(&mut number)
        .expect("Falied to read line");

    println!("The secret number is: {}", secrete_number);

    println!("Your number is {}", number)
}
