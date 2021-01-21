use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess Number");

    println!("Please enter your number: ");

    let secrete_number = rand::thread_rng().gen_range(1, 101);

    let mut number = String::new();

    io::stdin()
        .read_line(&mut number)
        .expect("Falied to read line");

    let number: u32 = number.trim().parse().expect("Please type a number!");

    println!("The secret number is: {}", secrete_number);

    println!("Your number is {}", number);

    match number.cmp(&secrete_number) {
        Ordering::Less => println!("Too small ..."),
        Ordering::Greater => println!("Too big ..."),
        Ordering::Equal => println!("The numbers are equal ..."),
    }
}
