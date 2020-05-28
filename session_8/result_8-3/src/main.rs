use std::num::ParseIntError;

fn main() {
    // Without Result
    let r = "100".parse::<i32>().unwrap();
    // let r = "XXX".parse::<i32>().unwrap();
    println!("{}", r);

    // Result
    // let r = "999".parse::<i32>();
    let r = "XXX".parse::<i32>();
    match r {
        Ok(n) => println!("n is {}", n),
        Err(e) => println!("error: {:?}", e),
    }

    // match half_number("XXXX") {
    match half_number("8080") {
        Ok(n) => println!("Ok: {}", n),
        Err(err) => println!("Error: {}", err),
    }
}

// fn returns Result
fn half_number(s: &str) -> Result<i32, ParseIntError> {
    match s.parse::<i32>() {
        Ok(n) => Ok(n / 2),
        Err(err) => Err(err),
    }
}
