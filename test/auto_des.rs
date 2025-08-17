fn main() {
    foo(Bar("hello".to_string()));
    baz(Qux {
        inner: "world".to_string(),
    })
}

struct Bar<T>(T);

fn foo(Bar(b): Bar<String>) {
    println!("{b}");
}

struct Qux<T> {
    inner: T,
}

fn baz(Qux { inner }: Qux<String>) {
    println!("{inner}");
}
