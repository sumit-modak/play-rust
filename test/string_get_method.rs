// Fix the error
fn main() {
    let names = [String::from("Sunfei"), "Sunface".to_string()];
    
    // `Get` returns an Option<T>, it's safe to use
    let name0: &String = names.get(0).unwrap();

    // But indexing is not safe
    let _name1: &String = &names[1];

    println!("{}, {}", name0, _name1);
}

