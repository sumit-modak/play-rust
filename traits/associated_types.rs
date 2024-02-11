#[derive(Debug)]
struct Point {
    x: u64,
    y: u64,
    z: u64,
}

trait Iterator: std::fmt::Debug {
    type Output;
    fn next(&mut self) -> Option<Self::Output>;
}

impl Iterator for Point {
    type Output = u64;
    fn next(&mut self) -> Option<Self::Output> {
        Some(1)
    }
}

// also this implementation cant be performed because
// there is already an implementation exists for Point
// so this will be conflicting while calling a Point
// function which takes Self as an argument
// impl Iterator for Point {
//     type Output = u32;
//     fn next(&mut self) -> Option<Self::Output> {
//         Some(1)
//     }
// }

// associated types can't be stored in a vtable during
// dynamic dispatch so it must be explicitly mentioned
fn foo(p: &dyn Iterator<Output = u64>) {
    println!("{:?}", p);
}

fn main() {
    let p = Point {
        x: 12,
        y: 23,
        z: 34,
    };
    foo(&p);
}
