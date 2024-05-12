use core::{generate_password, options::PasswordOptions};
use std::{io::{self, Read}, num::ParseIntError};

/// The exit symbol of the program.
const QUIT_KEY: &str = "q";
/// The positive response symbol.
const YES_KEY: &str = "y";
/// The negative response symbol.
const NO_KEY: &str = "n";

fn main() {

    println!("This is a password generation program. Enter '{QUIT_KEY}' to exit.");

    println!("Enter the desired password length [4 <= input <= 128]:");
    let length = match read_number() {
        InputResult::Some(value) => value,
        InputResult::Quit => return,
    };
    
    println!("Should the password contain numbers: [Yes - {YES_KEY} or no - {NO_KEY}]: ");
    let numbers = match read_boolean() {
        InputResult::Some(value) => value,
        InputResult::Quit => return,
    };

    println!("Should the password contain the characters: [Yes - {YES_KEY} or no - {NO_KEY}]: ");
    let symbols = match read_boolean() {
        InputResult::Some(value) => value,
        InputResult::Quit => return,
    };

    println!("Should the password contain uppercase characters: [Yes - {YES_KEY} or no - {NO_KEY}]: ");
    let uppercase = match read_boolean() {
        InputResult::Some(value) => value,
        InputResult::Quit => return,
    };

    let opt = PasswordOptions { length, numbers, symbols, uppercase };
    let password = generate_password(opt);

    match password {
        Ok(password) => println!("\nGenerated password: {password}"),
        Err(error) => println!("\nAn error occurred while generating the password: {error}"),
    }

    println!("\nPress Enter to exit...");
    let _ = io::stdin().read(&mut [0]);
}

/// Read user input in number format.
fn read_number() -> InputResult<u8> {
    loop {
        let mut buf = String::new();
        let _ = io::stdin().read_line(&mut buf);
        let buf_lower = buf.trim().to_lowercase();

        if buf_lower == QUIT_KEY {
            return InputResult::Quit;
        }
        let parse_result: Result<usize, ParseIntError> = buf_lower.parse();
        match parse_result {
            Ok(value) => {
                if value < 4 {
                    println!("The entered number does not meet the minimum limit.\nPlease repeat the input... "); 
                    continue;
                }
                else if value > 128 {
                    println!("The entered number does not meet the maximum limit.\nPlease repeat the input... "); 
                    continue;
                }
                return InputResult::Some(value as u8);
            },
            Err(_) => { 
                println!("The input is not recognized.\nPlease repeat the input... ");
                continue;
            },
        }
    }
}

/// Read user input in bool format (yes / no)
fn read_boolean() -> InputResult<bool> {
    loop{
        let mut buf = String::new();
        let _ = io::stdin().read_line(&mut buf);
        match buf.trim().to_lowercase().as_str() {
            YES_KEY => { 
                return InputResult::Some(true);
             }
            NO_KEY => { 
                return InputResult::Some(false); 
            }
            QUIT_KEY => { 
                return InputResult::Quit; 
            }
            _ => {
                println!("The input is not recognized.\nPlease repeat the input... ");
                continue; }
        }
    }
}

/// The result of user input.
enum InputResult<T>{
    /// The input is recognized.
    Some(T),
    /// The input prompts you to exit the application.
    Quit,
}