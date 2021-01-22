use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess Number");

    do_job(100);
    destruct_object();
    loop_in();

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

fn destruct_object() {
    let array = [3; 5];

    println!("a[0] {}", array[0]);
    println!("a[1] {}", array[1]);

    for number in array.iter() {
        println!("Numebr is {}", number);
    }

    let duple: (i32, i64, String) = (10, 20, "ok".to_string());
    let (value_one, value_two, string_value) = duple;

    println!("n1 is {}", value_one);
    println!("n2 is {}", value_two);
    println!("val is {}", string_value);
}

fn do_job(x: i128) -> i32 {
    if x > 20 {
        return 10
    }

    20
}

fn loop_in() {
    for number in (1..10).rev() {
        println!("{}!", number);
    }

    println!("Going out");
}
