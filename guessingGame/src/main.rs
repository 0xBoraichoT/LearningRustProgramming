use rand::Rng;
use std::io::{self, Write};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    println!("🎯 Welcome to the Rust Guessing Game!");
    println!("Choose difficulty: easy (1-10), medium (1-50), hard (1-100)");

    let difficulty = loop {
        print!("Enter difficulty (easy/medium/hard): ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim().to_lowercase();

        match choice.as_str() {
            "easy" => break 10,
            "medium" => break 50,
            "hard" => break 100,
            _ => println!("❌ Invalid choice, try again."),
        }
    };

    let secret_number = rand::thread_rng().gen_range(1..=difficulty);
    println!("I have chosen a number between 1 and {}. Can you guess it?", difficulty);

    loop {
        print!("Your guess: ");
        io::stdout().flush().unwrap();

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).unwrap();

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("⚠️ Please enter a valid number.");
                continue;
            }
        };

        if guess < secret_number {
            println!("Too low! 📉");
        } else if guess > secret_number {
            println!("Too high! 📈");
        } else {
            println!("🎉 You got it! The number was {}", secret_number);
            confetti_boom();
            break;
        }
    }
}

fn confetti_boom() {
    let colors = [
        "\x1b[31m", // red
        "\x1b[32m", // green
        "\x1b[33m", // yellow
        "\x1b[34m", // blue
        "\x1b[35m", // magenta
        "\x1b[36m", // cyan
    ];
    let shapes = ["*", "+", "x", "o", "@", "#"];

    println!("\x1b[2J\x1b[H"); // Clear terminal
    for _ in 0..30 { // 30 frames of confetti rain
        for _ in 0..100 { // Each frame has 100 pieces of confetti
            let color = colors[rand::thread_rng().gen_range(0..colors.len())];
            let shape = shapes[rand::thread_rng().gen_range(0..shapes.len())];
            print!("{}{} ", color, shape);
        }
        println!("\x1b[0m");
        sleep(Duration::from_millis(100));
    }
    println!("\x1b[0m"); // Reset color
}
