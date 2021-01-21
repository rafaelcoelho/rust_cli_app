use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess Number");

    let secrete_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please enter your number: ");

        let mut number = String::new();

        io::stdin()
            .read_line(&mut number)
            .expect("Falied to read line");

        let number: u32 = match number.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Only number values are accepted ...");
                break;
            }
        };

        println!("Your number is {}", number);

        match number.cmp(&secrete_number) {
            Ordering::Less => println!("Too small ..."),
            Ordering::Greater => println!("Too big ..."),
            Ordering::Equal => {
                println!("The numbers are equal ...");
                break;
            }
        }
    }
}
