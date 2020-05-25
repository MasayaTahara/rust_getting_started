fn main() {
    // If basic
    let a = 10;
    let b = 20;
    if a == b {
        println!("a == b");
    } else if a > b {
        println!("a > b");
    } else {
        println!("a < b");
    }

    // return with if
    let x = if a < b { 1 } else { 0 };
    println!("{}", x);
}
