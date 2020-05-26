fn main() {
    let mut v = vec![1,2,3,4,5];
    println!("v.first is {:?}", v.first());
    println!("v.first is {}", v.first().unwrap());
    println!("v.last is {:?}", v.last());
    println!("v.last is {}", v.last().unwrap());
    
    // get with if let
    if let Some(n) = v.first() {
        println!("v.first() -> n is {}", n);
    }

    // remove
    println!("v.first is {:?}", v.first());
    println!("v.remove(0) is {}", v.remove(0));
    println!("v.first is {:?}", v.first());

    // insert
    v.insert(0, 10);
    println!("v.first is {:?}", v.first());
    v.insert(v.len(), 9999);
    println!("v.last is {:?}", v.last());
}
