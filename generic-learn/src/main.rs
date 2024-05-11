use std::char;
use sum::{sum1, sum2};

mod sum;

fn main() {
    let numbers = vec![10, 5, 10, 20, 5];
    println!("Numbers: {:?}", numbers);
    println!("Sum: {}", sum1(&numbers));

    println('-', 10);

    let numbers = vec![10.5, 5.25, 10.25, 20.0, 5.0];
    println!("Numbers: {:?}", numbers);
    println!("Sum: {}", sum1(&numbers));
}


fn println(symbol: char, count: u32) {
    for _ in 0..count {
        print!("{}", symbol)
    }
    println!();
}