fn main() {
    let c1 = || ();
    let c2 = || 2;
    foo(c1);
    foo(c2); // <-- error
}

fn foo(f: fn()) {
    f()
    // the semicolon can be ignored because f() returns () type
    // and foo(f: fn()) also returns unit type
}
