fn main() {
    let v = vec![1,2,3,4,5];
    // for
    println!("for is ");
    for i in &v {
        print!("{} ", i);
    }
    println!("");

    // for and iterator
    println!("for and iter is ");
    for i in v.iter() {
        print!("{} ", i);
    }
    println!("");

    // iterator and next
    let mut p = v.iter();
    println!("p is {:?}", p);
    println!("p.next() is {:?}", p.next());
    println!("p.next() is {:?}", p.next());
    println!("p.next() is {:?}", p.next());
    println!("p.next() is {:?}", p.next());
    println!("p.next() is {:?}", p.next());
    println!("p.next() is {:?}", p.next()); // out of range
    println!("p.next() is {:?}", p.next()); // out of range

    // loop
    println!("by loop");
    let mut p = v.iter();
    loop {
        let x = p.next();
        if x == None {
            break;
        }
        println!("x is {}", x.unwrap());
    }

    // while
    println!("by while");
    let mut p = v.iter();
    while let Some(x) = p.next() {
        println!("x is {}", x);
    }

    // map
    println!("map *10");
    let lst = v.iter().map(|x| x * 10);
    for i in lst {
        println!("i is {}", i);
    }
}
