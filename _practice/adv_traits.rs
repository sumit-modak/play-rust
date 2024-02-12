struct Container(i32, i32);

trait Contains<A, B> {
    fn contains(&self, _: &A, _: &B) -> bool;
    fn first(&self) -> i32;
    fn last(&self) -> i32;
}

impl Contains<i32, i32> for Container {
    fn contains(&self, num1: &i32, num2: &i32) -> bool {
        (&self.0 >= num1) && (&self.1 >= num2)
    }
    fn first(&self) -> i32 {
        self.0
    }
    fn last(&self) -> i32 {
        self.1
    }
}

fn difference<A, B, C: Contains<A, B>>(container: &C) -> i32 {
    container.last() - container.first()
}

fn main() {
    let container = Container(5, 12);

    println!(
        "Does container contain {} and {}: {}",
        &3,
        &10,
        container.contains(&3, &10)
    );
    println!("First number: {}", container.first());
    println!("Last number: {}", container.last());
    println!("The difference is: {}", difference(&container));
}
