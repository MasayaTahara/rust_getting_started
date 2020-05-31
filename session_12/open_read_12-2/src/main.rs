use std::fs::File;
use std::io::Read;

fn main() {
    let path = "sample.txt";
    println!("read all lines by buffer.");
    // let mut file = File::open(path).expect("file not found");
    if let Ok(mut file) = File::open(path) {
        let mut data = String::new();
        // file.read_to_string(&mut data).expect("read error");
        if let Ok(_) = file.read_to_string(&mut data) {
            println!("data is {}", data);
        }
    }
}
