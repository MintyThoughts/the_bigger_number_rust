use clap::{arg, Parser};
use std::io::{stdin, stdout, Write};

// Clap Derive ( this is the argument parser section )

/// Just select the ui and then input the numbers
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// UI to get
    #[arg(short, long, default_value = "tui")]
    cli_ui_tui_etc: String,
    /// First number to process
    #[arg(short, long, default_value_t = 1)]
    first_number: i128,
    /// Second number to process
    #[arg(short, long, default_value_t = 0)]
    second_number: i128,
}

fn main() {
    // this initiates a variable that has all the arguments in it
    let args = Args::parse();
    // this passes the ui argument tu the ui variable
    let ui = args.cli_ui_tui_etc.as_str();
    //this checks the ui variable and runs the correspondent ui
    if ui == "cli" {
        cli(args);
    } else if ui == "tui" {
        simple_tui();
    } else {
        println!("you did not selected a correct ui option")
    }
}

fn get_input() -> String {
    // This code gets input from the terminal and then returns it without spaces or enters
    let mut input = String::new();
    let _ = stdout().flush();
    stdin()
        .read_line(&mut input)
        .expect("Did not enter a correct string");
    if let Some('\n') = input.chars().next_back() {
        input.pop();
    }
    if let Some('\r') = input.chars().next_back() {
        input.pop();
    }
    input
}
fn input_to_integer(string: String) -> i128 {
    // This converts the string variable passed into a integer
    let number: i128 = string.parse().unwrap_or(0);
    number
}
fn return_the_bigger_number(a: i128, b: i128) -> i128 {
    // This function compares a and b and returns the bigger
    if a >= b {
        a
    } else {
        b
    }
}

fn simple_tui() {
    let a = {
        print!("Type the first number: ");
        let a_but_string = get_input();
        let a_but_actually_integer = input_to_integer(a_but_string);
        a_but_actually_integer
    };
    let b = {
        print!("Type the second number: ");
        let b_but_string = get_input();
        let b_but_actually_integer = input_to_integer(b_but_string);
        b_but_actually_integer
    };
    let awnser = return_the_bigger_number(a, b);
    println!("The bigger number is: {}", awnser)
}
fn cli(args: Args) {
    let number_one = args.first_number;
    let number_two = args.second_number;
    println!("The bigger number is: {}", {
        return_the_bigger_number(number_one, number_two)
    })
}
