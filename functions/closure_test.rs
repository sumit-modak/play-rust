fn main() {
    let mut s = "hello".to_string();
    let c = || {
        println!("{s}");
        // s = "world".to_string();
        // println!("{s}");
        let x = s;
        println!("{x}");
    };
    takes_fnmut(c);
}

fn takes_fn(f: impl Fn() -> ()) {
    f();
}

fn takes_fnmut(mut f: impl FnMut() -> ()) {
    f();
}

fn takes_fnonce(f: impl FnOnce() -> ()) {
    f();
}
