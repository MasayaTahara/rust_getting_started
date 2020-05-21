fn main() {
    println!("Hello, world!");

    // Type inference
    let name = "My name is..." ;
    let age = 100 ;
    let age2 = 100 ;
    println!("name: {}, age: {}", name, age);

    println!("added (i32): {}", add_i32(age, 32)) ;
    println!("added (isize): {}", add_isize(age2, 32)) ;

    // String
    let name2 = String::from("John");
    println!("{} {}", name, name2);

    let name3 = String::from("Lennon");
    let name4 = name2 + " " + &name3;
    println!("{}", name4);

    let name5 = "Bob!";
    let name6 = format!("{} {}", name, name5);
    println!("{}", name6)
}

fn add_isize( x : isize, y : isize ) -> isize {
    x + y
}

fn add_i32( x : i32, y : i32 ) -> i32 {
    x + y
}
