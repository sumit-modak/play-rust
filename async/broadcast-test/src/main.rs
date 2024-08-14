use std::time::Duration;
use tokio::sync::broadcast;

#[tokio::main]
async fn main() {
    let capacity = 8;
    let (tx, mut rx) = broadcast::channel::<usize>(capacity);
    let mut rx2 = tx.subscribe();

    let _ = tokio::join!(
        tokio::spawn(async move {
            loop {
                println!("Data: {}", rx.recv().await.unwrap());
            }
        }),
        tokio::spawn(async move {
            loop {
                println!("Value: {}", rx2.recv().await.unwrap());
            }
        }),
        start(tx.clone(), 6),
        start(tx.clone(), 9),
    );
}

async fn start(tx: broadcast::Sender<usize>, factor: usize) {
    let _ = tokio::spawn(async move {
        for i in 0..16 {
            tokio::time::sleep(Duration::new(1, 0)).await;
            println!("{} secs elapsed", i);
            let _ = tx.send(i * factor);
        }
    })
    .await;
}
