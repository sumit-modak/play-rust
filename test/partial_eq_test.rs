fn main() {
    #[derive(PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }

    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 1, y: 2 };
    assert!(p1 == p2); // Works because of PartialEq
}
