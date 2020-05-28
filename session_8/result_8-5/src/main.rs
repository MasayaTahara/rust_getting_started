use std::num::ParseIntError;

// Return with map
fn half_number(s: &str) -> Result<i32, ParseIntError> {
    s.parse::<i32>().map(|n| n/2)
}

fn main() {
    match half_number("XXXX") {
        Ok(n) => println!("Ok: {}", n),
        Err(err) => println!("Error: {}", err),
    }
}
