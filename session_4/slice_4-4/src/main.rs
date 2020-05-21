fn main() {
    let s = "hello rust world";

    // top
    let a = &s[0..1];
    println!("{}", a);

    // top 5
    let a = &s[0..5];
    println!("{}", a);

    // 0 is not necessary
    let a = &s[..5];
    println!("{}", a);

    // on the way
    let a = &s[6..(6+4)];
    println!("{}", a);

    // to end
    let a = &s[11..];
    println!("{}", a);

    // can not access over length
    // let len = s.len();
    // let a = &s[11..len];
    // let a = &s[11..(len+1)];
    // println!("{}", a);

    // Get all!
    let a = &s[..];
    println!("{}", a);
}
