use std::ops::Index;

struct IndexOffset<'a> {
    slice: &'a [String; 2],
    offset: usize,
}

impl<'a> Index<usize> for IndexOffset<'a> {
    type Output = String;
    fn index(&self, index: usize) -> &'a Self::Output {
        &self.slice[index]
    }
}

fn main() {
    println!("Hello, world!");
    let sts: [String; 2] = [String::new(), String::new()];
    let sl = &sts;
    let a = IndexOffset {
        slice: sl,
        offset: 0,
    };
    let a = a.index(0);
    // let next = {
    //     a
    // };
}
