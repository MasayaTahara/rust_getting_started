fn main() {
    // If you need to change value of variable,
    // you have to use "mut"
    // let x = 10;
    let mut x = 10;
    println!("x: {}", x);
    x = 20;
    println!("x: {}", x);

    // Shadowing
    let y = 100;
    println!("y: {}", y);
    let y = 200;
    println!("y: {}", y);
}
