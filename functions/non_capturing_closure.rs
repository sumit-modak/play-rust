fn main() {
    let c1 = || {
        let _x = 0;
    };
    let c2 = || 2;
    foo(c1);
    bar(c2);
}

fn foo(f: fn()) {
    f();
}

fn bar(f: fn() -> i32) {
    f();
}
