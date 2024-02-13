#[derive(Debug)]
struct Array<T, const N: usize> {
    data: [T; N],
}

fn main() {
    let arrays = [
        Array { data: [1, 2, 3] },
        Array { data: [4, 5, 6] },
        Array { data: [7, 8, 9] },
        // Array { data: [10] },
        // this will not be possible because other
        // array types are Array<i32, 3> and this
        // one is Array<i32, 1>
    ];

    println!("{:?}", arrays);
}
