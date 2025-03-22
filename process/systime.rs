fn main() {
    let mut x = std::time::SystemTime::now();
    println!("{x:?}");
    x += std::time::Duration::new(5, 0);
    println!("{x:?}");
}
