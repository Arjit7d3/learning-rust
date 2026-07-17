use std::io;

fn main() {
    let a: i32;
    let b: i32;

    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();

    a = input.trim_end().parse().unwrap();

    input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    b = input.trim_end().parse().unwrap();

    println!("{}", a + b);
}
