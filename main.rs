use std::io;

fn main() {
    let mut name: String = String::new();
    io::stdin().read_line(&mut name).unwrap();
    name = name.trim().to_string();

    let mut age: String = String::new();
    io::stdin().read_line(&mut age).unwrap();

    let age: u32 = age.trim().parse().unwrap();

    println!("Hi, {}! You are {} years old.", name, age);
}
