struct Point {
    a: i32,
    b: i32
}

fn main() {
    let num = Some(4);
    let y = 23;

    match num {
        Some(a @ 5) => println!("5"),
        Some(x) if y > 5 => println!("{} {}", x, y),
        Some(_) => (),
        None => ()
    }

    let p = Point { a: 7, b: 8 };
    match p {
        Point { a: aa @ 5, .. } => println!("{}", aa),
        Point { a: aa, b: bb } => println!("{} {}", aa, bb)
    }
}
