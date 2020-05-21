fn main() {
    let s = "これは多分問題ない";
    println!("{}", s);

    // slice is difficult...
    // let one = &s[0..5];
    let one = &s[0..6];
    println!("{}", one);

    // length of Japanese is number of bytes, not number of character
    let len = s.len();
    println!("{}", len);

    // Push is no problem
    // you can use .push_str and format!
    // Also, you can cast from &str to &String

    // This is how to use Japanse as slice
    let mut v : Vec<char> = Vec::new();
    for c in s.chars() {
        v.push(c);
    }
    // Get part of string as char
    let v = &v[3..];
    // Use it as &String again
    let mut s = String::new();
    for c in v {
        s.push(*c);
    }
    println!("{}", s);
}
