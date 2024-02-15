trait MyTrait<T>
where
    Self: std::fmt::Debug,
    T: std::fmt::Debug,
{
    fn divide(&self, other: &T) {
        // let r = self as f64 / other as f64;
        println!("{self:?} / {other:?}");
    }
}

impl<T: std::fmt::Debug> MyTrait<T> for u64 {}
impl<T: std::fmt::Debug> MyTrait<T> for f64 {}
impl<T: std::fmt::Debug> MyTrait<T> for i64 {}

fn run<T: std::fmt::Debug>(val1: &dyn MyTrait<T>, val2: &dyn MyTrait<T>) {
    val1.divide(val2);
}

fn main() {
    let v: Vec<&dyn MyTrait<f64>> = vec![&32u64, &3.5f64, &-32i64];
    for i in 0..3 {
        for j in 0..3 {
            run(v[i], v[j]);
        }
    }
}
