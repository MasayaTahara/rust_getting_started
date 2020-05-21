fn main() {
    // No problem
    let s = "test";
    // s is copyed
    let s2 = s;
    // so, s is still there
    println!("s: {}", s);
    println!("s2: {}", s2);

    // Move, not copy
    let t = String::from("test");
    // t is moved to t2 because String is not implemented copy
    let t2 = t;
    // t is not there, it cannot be borrowed!
    // println!("t: {}", t);
    println!("t2: {}", t2);
}
