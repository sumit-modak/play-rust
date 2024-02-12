struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: std::fmt::Debug + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {:?}", self.x);
        } else {
            println!("The largest member is y = {:?}", self.y);
        }
    }
}

#[derive(Debug, PartialOrd, PartialEq)]
struct Unit(i32);

struct Test(u32, u32, u32);

fn main() {
    let p2 = Pair::new(Unit(1), Unit(2));
    let p3 = Pair::new(Test(1, 2, 3), Test(3, 2, 1));

    p2.cmp_display();
    // this will not work because Test struct does not impls
    // Debug, PartialEq and PartialOrd traits which is required
    // to implement cmp_display function
    // p3.cmp_display();
}
