use std::io;

const C: f32 = 32.0;

/// Converting by number_choise
/// number_choise: 1 - celsius to fahrenheit
/// number_choise: 2 - fahrenheit to celsius
fn convert(temperature: f32, number_choise: u8) -> Option<f32> {
    match number_choise {
        1 => Some(c_to_f(temperature)),
        2 => Some(f_to_c(temperature)),
        _ => None,
    }
}

/// Converting celsius to fahrenheit temp
fn c_to_f(celssius_temp: f32) -> f32 {
    (celssius_temp * (9.0 / 5.0)) + C
}

/// Converting fahrenheit to celsius temp
fn f_to_c(fahrenheit_temp: f32) -> f32 {
    (fahrenheit_temp - C) * (5.0 / 9.0)
}

fn main() {
    
    println!("Temperature converter. \n (1) C to F \n (2) F to C");

    let mut user_choise = String::new();

    io::stdin().read_line(&mut user_choise)
        .unwrap();

    let number_choise = user_choise
        .trim()
        .parse::<u8>()
        .expect("Please type a number!");

    println!("Enter temperature: ");

    let mut temperature = String::new();

    io::stdin().read_line(&mut temperature).unwrap();

    let temperature = temperature
        .trim()
        .parse::<f32>()
        .expect("Please type a number!");

    match convert(temperature, number_choise) {
        Some(result) => println!("The result of convertion is : {result}"),
        None => println!("Inknown convertion requested!"),
    };
}