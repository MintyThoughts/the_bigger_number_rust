use std::io::{stdin, stdout, Write};

fn main() {
    println!("This program will help you know which number is bigger :)");
    // TODO: Simplify "let a" and "let b"
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
    let awnser = return_the_bigger_number(a,b);
    println!("The bigger number is: {}", awnser)


}

fn get_input() -> String {
    // This code gets input from the terminal and then returns it
    let mut input = String::new();
    let _=stdout().flush();
    stdin().read_line(&mut input).expect("Did not enter a correct string");
    if let Some('\n')=input.chars().next_back() {
        input.pop();
    }
    if let Some('\r')=input.chars().next_back() {
        input.pop();
    }
    input
}
fn input_to_integer(string: String) -> i128{
    // This converts the string variable passed into a integer
    let number:i128 = string.parse().unwrap_or(0);
    number
}
fn return_the_bigger_number(a:i128, b:i128) -> i128 {
    // This function compares a and b and returns the bigger
    if a>=b { a } else { b }
}
