trait Animal {}

struct Dog {}
struct Cat {}

impl Animal for Dog {}
impl Animal for Cat {}

fn return_animal(s: &str) -> &dyn Animal {
    match s {
        "dog" => &Dog {},
        "cat" => &Cat {},
        _ => panic!()
    }
}

fn main() {
    let animal1 = return_animal("dog");
    let animal2 = return_animal("cat");
}