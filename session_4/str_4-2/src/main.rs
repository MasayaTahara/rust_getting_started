fn main() {
    let s = "hello, Rust";
    println!("He said {}.", s);

    // get slice
    let one = &s[0..5];
    let two = &s[7..];
    println!("0..5: {}", one);
    println!("7..: {}", two);

    // get length
    let len = s.len();
    println!("len: {}", len);

    // push
    let mut s = String::new();
    s.push_str("Hello, ");
    s.push_str("world ");
    s.push_str("of Rust!");
    println!("{}", s);

    // or
    let one = "Hello,";
    let two = "world";
    let three = "of Rust!";
    let s = format!("{} {} {}", one, two, three);
    println!("{}", s);

    // &str is used for constant string,
    // &String is userd for variable string.
    // This is how to create &String
    let s = "I am &String".to_string();
    println!("{}", s);
    // or
    let s = String::from("I am &String, too");
    println!("{}", s);
}
