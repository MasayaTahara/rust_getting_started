fn main() {
    // Initialize with vec!
    let v = vec![1,2,3,4,5];
    // You can also get value of vector with "get"
    // returns Some
    println!("v.get(0): {:?}", v.get(0));
    println!("v.get(0).unwrap: {}", v.get(0).unwrap());
    // If index is out of range
    println!("v.get(10): {:?}", v.get(10));
    // println!("v.get(10).unwrap: {}", v.get(10).unwrap());

    // add and remove
    let mut v: Vec<i32> = Vec::new();
    // add
    v.push(1);
    v.push(2);
    v.push(3);
    println!("v.len(): {}", v.len());
    // remove
    println!("pop 1: {:?}", v.pop());
    println!("pop 1: {:?}", v.pop());
    println!("pop 1: {:?}", v.pop());
    println!("v.len(): {}", v.len());
}
