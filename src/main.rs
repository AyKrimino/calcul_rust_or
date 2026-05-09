use colored::*;
use terminal_size::{Width, Height, terminal_size};
use std::io::{self, Write};

enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
    Exit,
}

struct Calculator {
    terminal_width: usize,
    app_name: String,
    menu: Vec<String>,
}

impl Calculator {
    fn new(width: usize, name: String, menu: Vec<String>) -> Self {
        Self {
            terminal_width: width,
            app_name: name,
            menu,
        }
    }

    fn render_menu(&self) {
        self.pretty_print(&self.app_name);
        for m in self.menu.iter() {
            self.pretty_print(m);
        }
    }

    fn styled_message(text: &String) -> impl std::fmt::Display {
        text
            .bright_cyan()
            .bold()
            .on_black()
    }

    fn styled_error(text: &String) -> impl std::fmt::Display {
        text
            .red()
            .bold()
            .on_black()
    }

    fn pretty_print(&self, text: &String) {
        println!(
            "{:^width$}",
            Calculator::styled_message(text),
            width = self.terminal_width
        );
    }

    fn pretty_print_inline(&self, text: &String) {
        print!("{}", Calculator::styled_message(text));
    }

    fn pretty_print_error(&self, text: &String) {
        println!("{}", Calculator::styled_error(text));
    }

    fn read_number(&self, prompt: &String) -> f64 {
        loop {
            self.pretty_print(prompt);
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();

            match input.trim().parse::<f64>() {
                Ok(num) => return num,
                Err(_) => {
                    self.pretty_print_error(&String::from("Please enter a valid number"));
                }
            }
        }
    }

    fn get_two_numbers(&self) -> (f64, f64) {
        let a = self.read_number(&String::from("Enter first number:"));
        let b = self.read_number(&String::from("Enter second number:"));

        (a, b)
    }

    fn start_loop(&self) {
        loop {
            self.pretty_print_inline(&String::from("Enter your choice: "));

            io::stdout().flush().unwrap();

            let mut option = String::new();

            io::stdin().read_line(&mut option).expect("Failed to read line");

            let option: i32 = match option.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    self.pretty_print_error(&String::from("Please type a number."));
                    continue;
                },
            };

            match option {
                1 => {
                    let (a, b) = self.get_two_numbers();
                    self.pretty_print(&format!("{} + {} = {}", a, b, a + b));
                },
                2 => {
                    let (a, b) = self.get_two_numbers();
                    self.pretty_print(&format!("{} - {} = {}", a, b, a - b));
                },
                3 => {
                    let (a, b) = self.get_two_numbers();
                    self.pretty_print(&format!("{} * {} = {}", a, b, a * b));
                },
                4 => {
                    let (a, b) = self.get_two_numbers();
                    if b == 0.0 {
                        self.pretty_print_error(&String::from("Cannot divide by zero"));
                        continue;
                    }
                    self.pretty_print(&format!("{} / {} = {}", a, b, a / b));
                },
                5 => {
                    self.pretty_print(&String::from("Exit"));
                    break;
                },
                _ => {
                    self.pretty_print_error(&String::from("Please type a number between 1 and 5"));
                    continue
                },
            }
        }
    }
}

fn main() {
    const NAME: &str = "CALCULATOR PROGRAM";

    let width = if let Some((Width(w), Height(_))) = terminal_size() {
        w as usize
    } else {
        80
    };

    let menu = vec![
        String::from("1) Addition"),
        String::from("2) Subtraction"),
        String::from("3) Multiplication"),
        String::from("4) Division"),
        String::from("5) Exit"),
    ];

    let calculator = Calculator::new(width, String::from(NAME), menu.clone());

    calculator.render_menu();

    calculator.start_loop();
}
