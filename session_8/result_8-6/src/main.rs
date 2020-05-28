use std::num::ParseIntError;

// Error transfer
fn half_number(s: &str) -> Result<i32, ParseIntError> {
    let n = s.parse::<i32>()?;
    Ok(n/2)
}

fn main() {
    match half_number("6848") {
        Ok(n) => println!("Ok: {}", n),
        Err(err) => println!("Error: {:?}", err),
    }
}
