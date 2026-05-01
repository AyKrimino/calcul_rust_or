use colored::*;
use terminal_size::{Width, Height, terminal_size};
use std::io::{self, Write};

fn main() {
    const NAME: &str = "CALCULATOR PROGRAM";

    let width = if let Some((Width(w), Height(_))) = terminal_size() {
        w as usize
    } else {
        80
    };

    let menu = [
        NAME,
        "1) Addition",
        "2) Subtraction",
        "3) Multiplication",
        "4) Division",
        "5) Exit",
    ];

    for m in menu.iter() {
        println!(
            "{:^width$}\n",
            m
            .bright_cyan()
            .bold()
            .on_black(),
            width = width
        );
    }

    loop {
        print!(
            "{}",
            "Enter your choice: "
            .bright_cyan()
            .bold()
            .on_black(),

        );
        io::stdout().flush().unwrap();

        let mut option = String::new();

        io::stdin().read_line(&mut option).expect("Failed to read line");

        let option: i32 = match option.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{}","Please type a number".red());
                continue;
            },
        };

        match option {
            1 => {
                let (a, b) = get_two_numbers();
                println!("{}", format!("{} + {} = {}", a, b, a + b).bright_cyan().bold().on_black())
            },
            2 => {
                let (a, b) = get_two_numbers();
                println!("{}", format!("{} - {} = {}", a, b, a - b).bright_cyan().bold().on_black())
            },
            3 => {
                let (a, b) = get_two_numbers();
                println!("{}", format!("{} * {} = {}", a, b, a * b).bright_cyan().bold().on_black())
            },
            4 => {
                let (a, b) = get_two_numbers();
                if b == 0.0 {
                    println!("{}", "Cannot divide by zero".red());
                    continue;
                }
                println!("{}", format!("{} / {} = {}", a, b, a / b).bright_cyan().bold().on_black())
            },
            5 => {
                println!("{}", "Exit");
                break;
            },
            _ => {
                println!("{}","Please type a number between 1 and 5".red());
                continue
            },
        }
    }
}

fn get_two_numbers() -> (f64, f64) {
    use colored::*;
    use std::io::{self, Write};

    fn read_number(prompt: &str) -> f64 {
        loop {
            print!(
                "{}",
                prompt.bright_cyan().bold().on_black(),
            );
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();

            match input.trim().parse::<f64>() {
                Ok(num) => return num,
                Err(_) => {
                    println!(
                        "{}",
                        "Please enter a valid number"
                            .red()
                            .bold()
                            .on_black(),
                    );
                }
            }
        }
    }

    let a = read_number("Enter first number:");
    let b = read_number("Enter second number:");

    (a, b)
}
