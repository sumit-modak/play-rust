use std::future::Future;

async fn foo<Fut: Future<Output = usize>>(f: impl Fn(usize, usize) -> Fut) -> usize {
    f(3, 4).await
}

async fn bar(f: impl AsyncFn(usize, usize) -> usize) -> usize {
    f(3, 4).await
}

#[tokio::main]
async fn main() {
    let name = "hecker";
    let cfoo = |x: usize, y: usize| async move {
        println!("Hey {name}! your sum is: {}", x + y);
        x + y
    };

    let name = "newhecker";
    let bfoo = async move |x: usize, y: usize| {
        println!("Hey {name}! your sum is: {}", x + y);
        x + y
    };

    let _ = foo(cfoo).await; // ok
    let _ = bar(bfoo).await; // ok
    // let _ = foo(bfoo).await; // err
    // let _ = bar(cfoo).await; // ok
}
