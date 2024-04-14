struct Foo<'a> {
    hello: &'a str,
}

impl<'a> Foo<'a> {
    fn bar(&self) -> String {
        // there is no to_string method on Foo
        String::from(self.to_string())
    }
}

struct FooBar<'a> {
    hello: Foo<'a>,
}

struct Bar<'a> {
    world: FooBar<'a>,
}

fn main() {
    let foo = Foo { hello: "World!" };
    let bar = Bar {
        world: FooBar { hello: foo },
    };

    // error here
    bar.world = bar.world.hello.bar().as_str();
    // `bar.world` stores a FooBar
    // `bar.world.hello.bar().as_str()` returns a &str
}
