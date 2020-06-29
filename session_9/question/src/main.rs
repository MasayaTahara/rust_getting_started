use english_numbers::convert_all_fmt;

fn main() {
    // Q. english-numbersという、i64の値を英語の文字列に変換してくれるcrateがあります。
    // まずはCargo.tomlのDependenciesに
    // english-numbers = "0.3"
    // を追記して、このmain.rsを実行してみてください。
    let val_i64 = 64;
    println!("{}", convert_all_fmt(val_i64));
    // Sixty-Four と表示されたでしょうか。
    // この関数 convert_all_fmt は便利ですが、i64の値しか引数として受け取れません。
    // 新しく「convert_num_into_string」という関数を作成し、
    // ジェネリクスを使って、引数として以下の数値を表すいろいろな型を受け取って英語に変換してください。
    // ヒント: https://docs.rs/num-traits/0.2.8/num_traits/cast/trait.ToPrimitive.html

    let val_i32: i32 = 32;
    let val_isize: isize = 99;
    let val_u32: u32 = 32;
    let val_u64: u64 = 64;
    let val_usize: usize = 99;
}
