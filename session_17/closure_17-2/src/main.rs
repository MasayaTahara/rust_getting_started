fn main() {
    println!("call closure");
    let print_name = |name, age| {
        println!("name: {}, age: {}", name, age);
    };
    print_name("John", 49);

    println!("use return of closure");
    let format_name = |name, age| format!("name: {}, age: {}", name, age);
    println!("{}", format_name("Bob", 84));

    println!("use map");
    let a = [("John", 18), ("Bob", 59)];
    let b = a
        .iter()
        .map(|(name, age)| format!("name: {}, age: {}", name, age));
    for it in b {
        println!("{}", it);
    }
}
