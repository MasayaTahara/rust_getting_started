fn main() {
    // for basic
    let v = vec![10, 20, 30, 40, 50];
    print!("basic v: ");
    for i in &v {
        print!("{} ", i);
    }
    println!("");

    // iterator
    print!("iterator v: ");
    for i in v.iter() {
        print!("{} ", i);
    }
    println!("");

    // with index
    print!("with index v: ");
    for (i, x) in v.iter().enumerate() {
        print!("{}-{} ", i, x);
    }
    println!("");
}
