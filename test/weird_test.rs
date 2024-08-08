struct MutStr<'a, 'b> {
    s: &'a mut &'b str,
}

fn main() {
    let mut s = "hello";

    *MutStr { s: &mut s }.s = "world";
    // the above line can be written as
    // let m = MutStr { s: &mut s };
    // let *m.s = "world";

    println!("{s}");
}
