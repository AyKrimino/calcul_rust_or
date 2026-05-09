use colored::*;
use terminal_size::{Width, Height, terminal_size};
use std::io::{self, Write};

enum LoopControl {
    Continue,
    Exit,
}

enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
    Exit,
    Default,
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

    fn styled_message(text: &str) -> impl std::fmt::Display {
        text
            .bright_cyan()
            .bold()
            .on_black()
    }

    fn styled_error(text: &str) -> impl std::fmt::Display {
        text
            .red()
            .bold()
            .on_black()
    }

    fn pretty_print(&self, text: &str) {
        println!(
            "{:^width$}",
            Calculator::styled_message(text),
            width = self.terminal_width
        );
    }

    fn pretty_print_inline(&self, text: &str) {
        print!("{}", Calculator::styled_message(text));
    }

    fn pretty_print_error(&self, text: &str) {
        println!("{}", Calculator::styled_error(text));
    }

    fn read_number(&self, prompt: &str) -> f64 {
        loop {
            self.pretty_print(prompt);
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();

            match input.trim().parse::<f64>() {
                Ok(num) => return num,
                Err(_) => {
                    self.pretty_print_error("Please enter a valid number");
                }
            }
        }
    }

    fn get_two_numbers(&self) -> (f64, f64) {
        let a = self.read_number("Enter first number:");
        let b = self.read_number("Enter second number:");

        (a, b)
    }

    fn get_user_option(&self) -> i32 {
        self.pretty_print_inline("Enter your choice: ");

        io::stdout().flush().unwrap();

        let mut option = String::new();

        io::stdin().read_line(&mut option).expect("Failed to read line");

        let option: i32 = match option.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                self.pretty_print_error("Please type a number.");
                -1
            },
        };

        option
    }

    fn map_option_to_operation(option: i32) -> Operation {
        match option {
            1 => Operation::Add,
            2 => Operation::Subtract,
            3 => Operation::Multiply,
            4 => Operation::Divide,
            5 => Operation::Exit,
            _ => Operation::Default,
        }
    }

    fn execute_operation(&self, operation: &Operation) -> Result<LoopControl, String> {
        match operation {
            Operation::Add => {
                let (a, b) = self.get_two_numbers();
                self.pretty_print(&format!("{} + {} = {}", a, b, a + b));

                Ok(LoopControl::Continue)
            },
            Operation::Subtract => {
                let (a, b) = self.get_two_numbers();
                self.pretty_print(&format!("{} - {} = {}", a, b, a - b));

                Ok(LoopControl::Continue)
            },
            Operation::Multiply => {
                let (a, b) = self.get_two_numbers();
                self.pretty_print(&format!("{} * {} = {}", a, b, a * b));

                Ok(LoopControl::Continue)
            },
            Operation::Divide => {
                let (a, b) = self.get_two_numbers();
                if b == 0.0 {
                    return Err(String::from("Cannot divide by zero"));
                }
                self.pretty_print(&format!("{} / {} = {}", a, b, a / b));
                Ok(LoopControl::Continue)
            },
            Operation::Exit => {
                self.pretty_print(&String::from("Exit"));
                Ok(LoopControl::Exit)
            },
            Operation::Default => Err(
                String::from("Please type a number between 1 and 5")
            ),
        }
    }

    fn start_loop(&self) {
        loop {

            let option: i32 = self.get_user_option();

            let operation = Calculator::map_option_to_operation(option);

            match self.execute_operation(&operation) {
                Ok(LoopControl::Continue) => continue,
                Ok(LoopControl::Exit) => break,
                Err(err) => self.pretty_print_error(&err),
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
