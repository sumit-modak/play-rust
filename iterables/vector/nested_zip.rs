fn main() {
    let u: Vec<Vec<i32>> = vec![vec![-1, -1, -1], vec![1, 1, 1]];
    let v: Vec<Vec<i32>> = vec![vec![1, 12, 23], vec![104, 205, 306]];

    let x = std::iter::zip(u, v)
        .map(|(rx, ry)| {
            std::iter::zip(rx, ry)
                .map(|(cx, cy)| cx + cy)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<Vec<_>>>();

    println!("{x:?}");
}
