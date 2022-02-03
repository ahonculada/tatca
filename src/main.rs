use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let s: &[String] = &args[1..];
    println!("{:?}", s);
}
