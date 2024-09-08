use std::io::Read;

fn main() {
    print!("Enter your name: ");
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();

    // same over here
    // let mut s = Vec::new();
    // std::io::stdin().read(&mut s).unwrap();
}
