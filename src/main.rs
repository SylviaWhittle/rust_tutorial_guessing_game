// use the io library from std to allow input/output
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("guess number");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("input number");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!(
                    "{} is not an unsigned integer. input an unsigned integer.",
                    guess.trim()
                );
                continue;
            }
        };

        println!("you guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("correct");
                break;
            }
        }
    }
}
