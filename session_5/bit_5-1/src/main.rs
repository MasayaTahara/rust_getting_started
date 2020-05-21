fn main() {
    let a : u8 = 0b1111;
    let b : u8 = 0b0011;
    // AND
    println!("a & b is {}", a & b);
    // OR
    println!("a | b is {}", a | b);

    let a : u8 = 0x02;
    // Shift
    println!("a << 1 is {}", a << 1);
    println!("a >> 1 is {}", a >> 1);
}
