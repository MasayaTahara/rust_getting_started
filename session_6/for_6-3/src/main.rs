fn main() {
    // for with range
    print!("10 times: ");
    for i in 0..10 {
        print!("{}, ", i);
    }
    println!("");

    // break
    print!("5 times: ");
    for i in 0..10 {
        if i == 5 {
            break;
        }
        print!("{}, ", i);
    }
    println!("");

    // continue
    print!("odd number: ");
    for i in 0..10 {
        if i % 2 == 0 {
            continue;
        }
        print!("{}, ", i);
    }
    println!("");

    // while
    print!("even number: ");
    let mut i = 0;
    while i < 10 {
        i += 2;
        print!("{}, ", i);
    }
    println!("");

    // loop
    print!("many number: ");
    let mut i = 0;
    loop {
        i += 1;
        print!("{}, ", i);
        if i >= 20 {
            break;
        }
    }
    println!("");
}
