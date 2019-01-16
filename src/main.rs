use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess stuff!");

    let secret_number = rand::thread_rng().gen_range(1,101);

    loop {
            
        println!("Please input stuff.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Failed to read");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number");
                continue;
            }
        };

        println!("You guessed {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal => {
                println!("Yep....");
                break;
            }
        }
    }

}
