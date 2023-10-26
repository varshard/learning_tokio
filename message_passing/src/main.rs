use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (chan1, mut rx) = mpsc::channel(32);
    let chan2 = chan1.clone();

    tokio::spawn(async move {
        let _ = chan1.send("t1").await;
    });
    tokio::spawn(async move {
        let _ = chan2.send("t2").await;
    });

    tokio::select! {

    }
    // just like Go channel with a loop select
    while let Some(msg) = rx.recv().await {
        println!("GOT = {}", msg);
    }
}