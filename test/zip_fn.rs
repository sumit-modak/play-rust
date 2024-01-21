fn main() {
    let names = ["foo", "bar", "baz"];
    let ages = [1, 2, 3];
    let zipped: Vec<_> = names.iter().zip(ages.iter()).collect();
    println!("{:?}", zipped);
}
