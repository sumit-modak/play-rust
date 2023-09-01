fn main() {
    let x: i32 = 5;
    let y: &i32 = &x;
    let z: &&i32 = &y;
    println!("{}, {:p}, {:p}", x, y, z);
}
