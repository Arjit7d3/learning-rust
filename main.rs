use std::io;

fn main() {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input = input.trim().to_uppercase();
    println!("{}", input);
}
