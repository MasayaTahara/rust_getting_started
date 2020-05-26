fn main() {
    // concat
    let a = vec![1,2,3];
    let b = vec![4,5];
    let v = [a,b].concat();
    println!("v.len() is {}", v.len());
    for i in v {
        print!("{} ", i);
    }
    println!("");

    // 文字で連結
    let v = vec!["one", "two", "three", "four", "five"];
    let x = v.join("-");
    println!("x is {}", x);

    // 文字で分割
    let s = "one, two, three, four, five";
    let v = s.split(", ");
    for x in v {
        print!("{} ", x);
    }
    println!("");

    // sort
    let mut v = vec!["one", "two", "three", "four", "five"];
    v.sort();
    let x = v.join(" ");
    println!("x is {}", x);

    v.reverse();
    let x = v.join(" ");
    println!("x is {}", x);
}
