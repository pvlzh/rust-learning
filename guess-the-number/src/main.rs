use std::io::{self, Read};
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng()
        .gen_range(1..=100);
    
    loop{
        let mut input = String::new();  
        
        println!("Please input your guess.");
        
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");

        let parsed_value: u32 = match input.trim()
            .parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please type a number!");
                    continue;
                },
            };
            
            match parsed_value.cmp(&secret_number) {
                Ordering::Less => println!("Too small!"),
                Ordering::Greater => println!("Too big!"),
                Ordering::Equal => { 
                    println!("You win!"); 
                    break; 
                },
            };
        }
        
        
    println!("Press 'Enter' for closing application.");
    _ = io::stdin().read(&mut [0]);
}