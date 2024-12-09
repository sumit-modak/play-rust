fn main() {
    let x = 1.8273;
    let y = 1;
    let width = 8;
    println!("{x:0<width$}");
    println!("{y:0<width$}");
    println!("{y:.2}");
    println!("{x:.8}");
    println!("{x:0<width$.2}");
}
