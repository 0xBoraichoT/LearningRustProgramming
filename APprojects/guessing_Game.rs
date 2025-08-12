use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Welcome to the Number Guessing Game!");
    
    // Select difficulty level
    let (max_number, max_attempts) = select_difficulty();
    
    let secret_number = rand::thread_rng().gen_range(1..=max_number);
    let mut attempts = 0;
    
    println!("I'm thinking of a number between 1 and {}...", max_number);
    println!("You have {} attempts to guess it!", max_attempts);
    
    loop {
        println!("Please input your guess:");
        
        let mut guess = String::new();
        
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
            
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };
        
        attempts += 1;
        let remaining_attempts = max_attempts - attempts;
        
        match guess.cmp(&secret_number) {
            Ordering::Less => {
                give_temperature_hint(guess, secret_number);
                println!("Too small! {} attempts remaining.", remaining_attempts);
            }
            Ordering::Greater => {
                give_temperature_hint(guess, secret_number);
                println!("Too big! {} attempts remaining.", remaining_attempts);
            }
            Ordering::Equal => {
                println!("You win! You guessed the number in {} attempts!", attempts);
                break;
            }
        }
        
        if attempts >= max_attempts {
            println!("Game over! You've used all your attempts. The number was {}.", secret_number);
            break;
        }
    }
}

fn select_difficulty() -> (u32, u32) {
    loop {
        println!("Select difficulty level:");
        println!("1 - Easy (1-10, 7 attempts)");
        println!("2 - Medium (1-50, 7 attempts)");
        println!("3 - Hard (1-100, 7 attempts)");
        
        let mut choice = String::new();
        
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");
            
        match choice.trim() {
            "1" => return (10, 7),
            "2" => return (50, 7),
            "3" => return (100, 7),
            _ => println!("Please enter 1, 2, or 3"),
        }
    }
}

fn give_temperature_hint(guess: u32, secret: u32) {
    let difference = if guess > secret {
        guess - secret
    } else {
        secret - guess
    };
    
    let range = match secret {
        1..=10 => 2,    // For easy mode
        1..=50 => 10,   // For medium mode
        _ => 20,        // For hard mode
    };
    
    if difference <= range / 5 {
        println!("🔥 Boiling hot!");
    } else if difference <= range / 2 {
        println!("♨️ Warm!");
    } else if difference <= range {
        println!("❄️ Cold!");
    } else {
        println!("🧊 Ice cold!");
    }
}
