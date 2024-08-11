use tokio::sync::oneshot;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = oneshot::channel::<usize>();

    let _ = tokio::spawn(async move {
        tokio::time::sleep(std::time::Duration::new(2, 0)).await;
        println!("tx: successfully slept for 2 secs");
        tx.send(7).unwrap();
        // tokio::time::sleep(std::time::Duration::new(2, 0)).await;
        // println!("tx: successfully slept for another 2 secs");
        // tx.send(8).unwrap();
    })
    .await;

    let _ = tokio::spawn(async move {
        tokio::time::sleep(std::time::Duration::new(2, 0)).await;
        println!("rx: successfully slept for 2 secs");
        let r = rx.try_recv();
        println!("{r:?}");
        let r = rx.try_recv();
        println!("{r:?}");
    })
    .await;
}
