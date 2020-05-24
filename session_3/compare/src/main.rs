fn main() {
    println!("Hello, world!");

    // 引数
    let a = 1;
    let b = 2;
    println!("{}", compare(a, b));
}

fn compare(a: isize, b: isize) -> isize {
    // a > b ? { a } : { b }
    // 三項演算子はない

    if a > b {
        a
    } else {
        b
    }
}
