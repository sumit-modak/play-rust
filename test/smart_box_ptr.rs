fn main() {
    let x: i16 = 5;
    let y: Box<i16> = Box::new(x);
    let z: Box<Box<i16>> = Box::new(y);
    std::mem::drop(x);
    println!("{} {}", y, z);  // *y *z
}