use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main() {
    print!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Your secret number is {}", secret_number);

    loop {
        println!("Type your guess below");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        println!("Your guess is {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}
