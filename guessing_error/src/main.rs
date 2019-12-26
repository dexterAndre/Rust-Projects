use std::io;
use std::cmp::Ordering;
use rand::Rng;



pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Error: The \"value\" field of \"Guess\" is ∉ [1, 100].");
        }

        return Guess { value };
    }

    pub fn value(&self) -> i32 {
        return self.value;
    }
}

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess...");
        
        let mut guess = String::new();
        
        io::stdin().read_line(&mut guess)
            .expect("Error: Failed to read line.");

        // Cleaning input
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // Error handling
        if guess < 1 || guess > 100 {
            println!("Error: The secret number is ∈ [1, 100]");
            continue;
        }

        // Guess logic
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too low."),
            Ordering::Greater => println!("Too high."),
            Ordering::Equal => {
                println!("Correct!");
                break;
            },
        }
    }
}
