fn main() {
    // match with number
    let x = 2;
    let m = match x {
        1 => "one",
        2 => "two",
        3 => "three",
        _ => "other",
    };
    println!("x -> m: {}", m);

    let x: usize = 100;
    let m = match x {
        0..=3 => "under 3",
        4..=10 => "4-10",
        _ => "over 10",
    };
    println!("x -> m: {}", m);

    // match with char
    let x = 'A';
    let m = match x {
        'A' => 1,
        'B' => 2,
        'C' => 3,
        _ => -1,
    };
    println!("x -> m: {}", m);

    // match with &str
    let x = "two";
    let m = match x {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        _ => -1,
    };
    println!("x -> m: {}", m);
}
