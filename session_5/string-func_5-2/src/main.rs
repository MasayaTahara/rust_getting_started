fn main() {
    let ret = str_param_and_return("rust");
    println!("{}", ret)
}

fn str_param_and_return( s: &str ) -> String {
    // Returning new variables is better than changing parameter
    let ret = format!("hello, {} world!", s);
    ret
}
