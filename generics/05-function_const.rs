fn print_array<T: std::fmt::Debug, const N: usize>(arr: T) {
    println!("{:?}", arr);
}

fn main() {
    let arr = [1, 2, 3];
    print_array::<[i32; 3], 3>(arr);

    let arr = ["hello", "world"];
    print_array::<[&str; 2], 2>(arr);
}

