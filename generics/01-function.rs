fn sum<T: std::ops::Add<Output = T>>(x: T, y: T) -> T {
    x + y
}

fn main() {
    println!("{} == {}", 5, sum(2i8, 3i8));
    println!("{} == {}", 50, sum(20, 30));
    println!("{} == {}", 2.46, sum(1.23, 1.23));
}

