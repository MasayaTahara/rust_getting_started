fn main() {
    let v = vec![1, 2, 3, 4, 5];
    let sum = vec_param(&v);
    println!("{}", sum);

    let v = vec_return( 20 );
    for i in v {
        println!("i: {}", i);
    }

    let v = vec![1, 2, 3, 4, 5];
    for i in v {
        println!("i: {}", i);
    }
    let mut v = vec![1, 2, 3, 4, 5];
    vec_cahnge(&mut v);
    for i in v {
        println!("i: {}", i);
    }}

fn vec_param( v: &Vec<i32> ) -> i32 {
    let mut sum = 0;
    for i in v {
        sum += i;
    }
    sum
}

fn vec_return( max: i32 ) -> Vec<i32> {
    let mut v = Vec::new();
    for i in 0..max {
        v.push( i );
    }
    v
}

fn vec_cahnge( v: &mut Vec<i32> ) {
    for i in v {
        *i = *i + 10;
    }
}
