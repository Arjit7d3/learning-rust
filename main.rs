use std::io;

fn main() {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let num: u32 = input.trim().parse().unwrap();

    if num % 15 == 0 {
        println!("FizzBuzz");
    } else if num % 3 == 0 {
        println!("Fizz");
    } else if num % 5 == 0 {
        println!("Buzz");
    } else {
        println!("{}", num);
    }
}
