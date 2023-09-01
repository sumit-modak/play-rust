use std::io;

fn main() {
    let x = 7;
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let y = buffer.parse::<i32>().unwrap();
    println!("{} {}", x, y);
}