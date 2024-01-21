extern crate bit_set;
use bit_set::BitSet;

fn main() {
    let n = 5;
    let b = BitSet::with_capacity(n);
    b.insert(2);
    println!("{:?}", b);
}
