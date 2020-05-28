// function with original Result
fn half_number(s: &str) -> Result<i32, &str> {
    match s.parse::<i32>() {
        Ok(n) => Ok(n/2),
        Err(_err) => Err("数値ではありません！"),
    }
}

fn main() {
    match half_number("XXXXX") {
        Ok(n) => println!("Ok: {}", n),
        Err(err) => println!("Err: {}", err),
    }
}
