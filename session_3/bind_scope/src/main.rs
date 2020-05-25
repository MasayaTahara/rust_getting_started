fn main() {
    // Q.1
    // 2 ways to pass compile
    let a = 1;
    println!("{}", a);
    a = 2;
    println!("{}", a);

    // Q.2
    // Q.2-1
    let a = 10;
    fn add_a_1 (x: isize) -> isize {
        x + a
    }
    println!("{}", add_a_1(1));
    // Q.2-1
    let add_a_2 = |x| { x + a };
    println!("{}", add_a_2(1));
}
