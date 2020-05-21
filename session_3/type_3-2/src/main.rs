fn main() {
    // Tuple
    let t = ("John", 123);
    println!("name: {}, age: {}", t.0, t.1);

    // Array
    let season = ["Spring", "Summer", "Fall", "Winter"];
    println!("1: {}", season[0]);
    println!("2: {}", season[1]);
    println!("3: {}", season[2]);
    println!("4: {}", season[3]);

    let n = 4;
    println!("Error: {}", season[n]);
}
