#[derive(Debug)]
enum LANG {
    JAPANESE,
    ENGLISH,
    CHINESE,
    FRENCH,
}

fn main() {
    let ja = LANG::JAPANESE;
    // println!("ja: {}", ja as i32);
    println!("ja: {:?}", ja);

    // match
    let lang = LANG::JAPANESE;
    let m = match lang {
        LANG::JAPANESE => "日本語",
        LANG::ENGLISH => "英語",
        _ => "その他の言語",
    };
    println!("lang is {}", m);
}
