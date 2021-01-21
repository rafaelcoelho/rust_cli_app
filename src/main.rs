use std::io;

fn main() {
    println!("Guess Number");

    println!("Please enter your number: ");

    let mut number = String::new();

    io::stdin()
    .read_line(&mut number)
    .expect("Falied to read line");

    println!("Your number is {}", number)
}
