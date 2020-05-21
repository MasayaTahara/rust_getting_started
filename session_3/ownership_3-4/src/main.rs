fn main() {
    let x = String::from("Hello") ;
    let len = string_length( x );
    println!("len is {}", len);
    // Error, because x is borrowed, not there
    // println!("x is {}", x);

    let x2 = String::from("Hello!!") ;
    // let len = string_length_2( &x2 );
    let len = string_length_3( &x2 );
    println!("len is {}", len);
    // x2 is there, because "string_length_2"'s parameter is reference type
    println!("x2 is {}", x2);
}

fn string_length(s: String) -> usize {
    let length = s.len();
    length
}

fn string_length_2(s: &String) -> usize {
    let length = s.len();
    length
}

// also, no problem
fn string_length_3(s: &str) -> usize {
    let length = s.len();
    length
}
