fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let n = s
        .split_whitespace()
        .next()
        .unwrap()
        .parse::<usize>()
        .unwrap();

    let x = Box::new([0; n]);
    println!("{:?}", x);
}
