fn main() {
    let ch = 'D';
    println!("ch: {}", ch);
    // Cast from char to u8
    let u = ch as u8;
    println!("u: {}", u);
    let ch = u as char;
    println!("ch: {}", ch);
}
