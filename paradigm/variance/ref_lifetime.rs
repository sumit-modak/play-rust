fn main() {
    let s1: &'static str = "hello";
    {
        let s2 = String::from("world");
        foo(&s2);
        // bar(&s2);
        // since `s2` is getting dropped at the end of this
        // scope so `s2` is not considered static and `bar`
        // always takes a &'static str so this will be an error
        // where compiler says the str doesn't live long enough
    }
    foo(s1);
    bar(s1);
}

fn foo<'a>(s: &'a str) {
    println!("{}", s);
}

fn bar(s: &'static str) {
    println!("{}", s);
}

// so `&'a T` and `&'a mut T` are covariant over lifetime
// which means that you can use longer lifetime('static or
// something bigger) on any specified lifetime('a) which is shorter

// mut is not implemented here because the whole thing will
// be the exact same
