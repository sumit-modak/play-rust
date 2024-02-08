fn main() {
    let x = 32;
    let s = "hello".to_string();
    let c1 = || {
        let _ = x;
    };
    let c2 = || {
        let _ = s;
    };
    bar(c1); // cannot pass because c1 is a capturing closure
    bar(c2); // cannot pass because c2 is a capturing closure
}

fn bar(f: fn()) {
    f()
}
