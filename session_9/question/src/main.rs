fn same_u8(a: u8, b: u8) -> String {
    if a == b {
        "Yes".to_string()
    } else {
        "No".to_string()
    }
}

fn main() {
    // 以下の関数 same_u8 を見てください。
    let a: u8 = 1;
    let b: u8 = 1;
    println!("{}", same_u8(a, b));
    // u8型の変数を2つ渡すと、それらが同じときに"Yes"、異なる時に"No"を返します。
    // ですが、このままではusize型やi64型の変数は渡すことができません。

    // Q. u64でも、isize型でも、文字列でも、
    // same_u8関数と同様に2つの引数が同じであれば"Yes"、異なる場合に"No"を返す関数"same"を作ってください。
}
