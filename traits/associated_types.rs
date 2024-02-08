trait Iterator {
    type Output;

    fn next(&mut self) -> Option<Self::Output>;
}

struct Point {
    x: u64,
    y: u64,
    z: u64,
}

impl Iterator for Point {
    type Output = u64;

    fn next(&mut self) -> Option<Self::Output> {
        Some(1)
    }
}
