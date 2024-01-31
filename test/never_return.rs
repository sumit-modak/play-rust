fn main() {
    f(0, 1);
}

fn f(a: u8, b: u8) {
    f(b, a);
}
