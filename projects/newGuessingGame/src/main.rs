use rand::Rng;
use std::cmp::Ordering;
use std::io;
use colored::*;
use std::{thread, time};

fn main() {
    println!("🎮 Welcome to the Guessing Game!");

    // Choose difficulty level
    let (max, difficulty) = choose_difficulty();

    println!(
        "\nYou chose {} mode. Guess the number between 1 and {}!",
        difficulty, max
    );

    let secret_number = rand::thread_rng().gen_range(1..=max);
    let mut attempts = 0;

    loop {
        println!("\nPlease input your guess:");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("❌ Please enter a valid number!");
                continue;
            }
        };

        attempts += 1;

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("🔻 Too small! {}", hot_or_cold(guess, secret_number));
            }
            Ordering::Greater => {
                println!("🔺 Too big! {}", hot_or_cold(guess, secret_number));
            }
            Ordering::Equal => {
                println!("\n✅ You guessed the number in {} attempts!", attempts);
                print_confetti();
                break;
            }
        }
    }
}

fn choose_difficulty() -> (u32, &'static str) {
    println!("\nChoose a difficulty level:");
    println!("1. Easy   (1 - 10)");
    println!("2. Medium (1 - 50)");
    println!("3. Hard   (1 - 100)");

    loop {
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => return (10, "Easy"),
            "2" => return (50, "Medium"),
            "3" => return (100, "Hard"),
            _ => {
                println!("❌ Invalid choice. Please enter 1, 2, or 3.");
                continue;
            }
        }
    }
}

fn hot_or_cold(guess: u32, target: u32) -> String {
    let distance = (guess as i32 - target as i32).abs();

    match distance {
        0 => "🎯".to_string(),
        1..=3 => "🔥 You're very hot!".red().to_string(),
        4..=10 => "🌡️ Getting warmer...".yellow().to_string(),
        _ => "❄️ Cold!".blue().to_string(),
    }
}

fn print_confetti() {
    let width = 80;
    let height = 20;
    let chars = vec!['*', '+', '@', '#', '%', '$', '&', '🎉', '🎊'];

    println!("\n🎉🎉🎉 YOU WIN! 🎉🎉🎉\n");

    for _ in 0..height {
        let mut line = String::new();
        for _ in 0..width {
            let ch = chars[rand::thread_rng().gen_range(0..chars.len())];
            let colored_ch = match rand::thread_rng().gen_range(0..6) {
                0 => ch.to_string().red(),
                1 => ch.to_string().green(),
                2 => ch.to_string().yellow(),
                3 => ch.to_string().blue(),
                4 => ch.to_string().magenta(),
                _ => ch.to_string().cyan(),
            };
            line.push_str(&colored_ch.to_string());
        }
        println!("{}", line);
        thread::sleep(time::Duration::from_millis(50));
    }

    println!("\n{}", "🎊 YOU NAILED IT! 🎊".bold().bright_white().on_bright_green());
}
