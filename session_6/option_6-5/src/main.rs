fn main() {
    // let x = Some(10);
    let x = None;
    let v = match x {
        Some(i) => i,
        None => -1,
    };
    println!("x -> v: {}", v);

    // from match to if let
    let x = Some(10);
    // 安全な取り出し
    if let Some(i) = x {
        println!("x -> i: {}", i);
    }
}
