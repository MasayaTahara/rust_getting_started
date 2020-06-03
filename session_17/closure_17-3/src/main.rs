fn call_with_one<F>(x: usize, func: F) -> usize
where
    F: Fn(usize) -> usize,
{
    func(x)
}

fn call_with_vec<F>(v: &Vec<usize>, func: F) -> usize
where
    F: Fn(usize) -> usize,
{
    let mut sum = 0;
    for i in v {
        sum += func(*i);
    }
    sum
}

fn main() {
    let double = |x| x * 2;
    let triple = |x| x * 3;
    let a = call_with_one(99, double);
    let b = call_with_one(99, triple);
    println!("a is {}", a);
    println!("b is {}", b);

    let v = vec![1, 2, 3, 4, 5];
    let a = call_with_vec(&v, double);
    println!("a is {}", a);
    let sum: usize = v.iter().map(|x| x * 2).sum();
    println!("and also {}", sum);
}
