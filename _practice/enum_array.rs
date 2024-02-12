#[derive(Debug)]
enum Boolean {
    True,
    False,
}

fn main() {
    let a: [Boolean; 2] = [Boolean::True, Boolean::False];
    println!("{:?}", a);
}
