#[derive(Debug)]
struct Person {
    name: String,
    age: i32,
}

fn main() {
    let x: Person;
    {
        let a = Person {
            name: String::from("Bob"),
            age: 48,
        };
        x = a;
        // x = &a;
    }
    println!("x is {:?}", x);
}
