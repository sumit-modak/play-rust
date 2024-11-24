struct Val<T> {
    val: T,
}

enum Hello {
    Some(u8),
    None,
}

impl<T> Val<T> {
    #[inline]
    fn value(&self) -> &T {
        &self.val
    }
}

fn main() {
    let s = "\"hello\"";
    let x = Val { val: 3.0 };
    let y = Val {
        val: "hello".to_string(),
    };
    println!("{}, {}", x.value(), y.value());
}
