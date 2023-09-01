fn main() {
    let mut x = 0;
    change(&mut x);
    println!("{}", x);
    let y: &i32;
}

fn change(x: &mut i32) {
    *x += 1;
}