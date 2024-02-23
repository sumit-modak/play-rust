fn main() {
    let mut s1: &'static str = "hello";
    {
        let s2 = String::from("world");
        foo(&mut (&s2 as &str));
        // bar(&mut (&s2 as &str));
        // `s2` cant be passed here because `&'a mut T` shows
        // invariance, which means that you need to pass exact
        // lifetime for T which is specified in the given function
    }
    foo(&mut s1);
    bar(&mut s1);
}

fn foo<'a>(s: &mut &'a str) {
    println!("{s}");
}

fn bar(s: &mut &'static str) {
    println!("{s}");
}
