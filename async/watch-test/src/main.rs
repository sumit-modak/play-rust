use std::time::Duration;
use tokio::sync::watch::{self, Sender};

#[tokio::main]
async fn main() {
    let (tx, mut rx) = watch::channel::<usize>(usize::MAX);
    let mut rx1 = rx.clone();

    let _ = tokio::join!(
        start(tx),
        tokio::spawn(async move {
            loop {
                println!("Value: {}", *rx.borrow_and_update());
                if rx.changed().await.is_err() {
                    break;
                }
            }
        }),
        tokio::spawn(async move {
            loop {
                println!("Data: {}", *rx1.borrow_and_update());
                if rx1.changed().await.is_err() {
                    break;
                }
            }
        })
    );
}

async fn start(tx: Sender<usize>) {
    for i in 0..16 {
        let tx = tx.clone();
        let _ = tokio::spawn(async move {
            // for every iteration its awaiting i secs
            // tokio::time::sleep(Duration::new(i, 0)).await;
            tokio::time::sleep(Duration::new(1, 0)).await;
            println!("{} secs elapsed", i);
            let _ = tx.send(i as usize);
        })
        .await;
    }
}
