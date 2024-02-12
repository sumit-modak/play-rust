#[derive(Debug)]
struct SomeStruct {
    num: i32,
}

// the lifetime of the return type depends on the arguments passed
// to the function and in this case rust compiler cant figure out
// the lifetime dependency of return type
fn biggest<'a>(a: &'a SomeStruct, b: &'a SomeStruct) -> &'a SomeStruct {
    if a.num > b.num {
        a
    } else {
        b
    }
}

fn main() {
    let some_struct = SomeStruct { num: 3 };
    let other_struct = SomeStruct { num: 5 };
    let bigger = biggest(&some_struct, &other_struct);

    println!("{:?}", bigger);
}
