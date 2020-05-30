#[derive(Debug)]
struct Person {
    name: String,
    age: i32,
}

fn main() {
    let mut a = Person {
        name: String::from("John"),
        age: 75,
    };
    println!("a is {:?}", a);

    // let mut x = &mut a ;
    // let mut y = &mut a ;
    // x.age = 0;
    // y.name = String::from("Bob");
    // println!("a is {:?}", a);
    // println!("x is {:?}", x);
    // println!("y is {:?}", y);

    let mut x = &mut a ;
    x.age = 0;
    println!("x is {:?}", x);
    let mut y = &mut a ;
    y.name = String::from("Bob");
    println!("y is {:?}", y);
    println!("a is {:?}", a);
}
