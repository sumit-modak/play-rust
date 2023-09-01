fn main() {
    // let x: [char; 3] = ['a', 'b', 'c'];
    let y: String = "abc".to_string();
    let z: &str = &y[..];
    // assert_eq!(x, y);  // panics
    assert_eq!(y, z);
}
