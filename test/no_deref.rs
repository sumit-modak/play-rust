fn main() {
    let x: i32 = 7;
    let mut y: &i32 = &x;
    println!("{} {}", x, y);
    y = &8;     // y value changed
    // y += 1;      // Error! cannot perform operation 
    let z: i32 = y + 4;
    println!("{} {}", y, z);
}