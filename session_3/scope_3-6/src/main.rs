fn main() {
    // function scope
    let x = 55;
    println!("test: {}", test(x));
    println!("test2: {}", test2(x));
    println!("test3: {}", test3(x));

    // Closure
    let num = 10;
    let add_one = |x| { num + x };
    let add_two = |x, y| { x + y };

    let ans = add_one(1);
    println!("ans: {}", ans);
    let ans = add_two(10, 20);
    println!("ans: {}", ans);

    // Error
    // fn add(x: i32) -> i32 {
    //     num + x
    // }
}

fn test(x: i32) -> i32 {
    // If you can, do not use "mut"
    let mut ans = x;
    if x < 0 {
        ans = 0;
    }
    if x > 100 {
        ans = 100;
    }
    ans
}

fn test2(x: i32) -> i32 {
    // better
    if x < 0 {
        0
    } else if x > 100 {
        100
    } else {
        x
    }
}

fn test3(x: i32) -> i32 {
    // longer, but easy to read
    let ans = if x < 0 {
        0
    } else if x > 100 {
        100
    } else {
        x
    };
    ans
}
