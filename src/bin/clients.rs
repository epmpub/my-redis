use tokio::sync::mpsc;


#[tokio::main]
async fn main() {

    let (tx,mut rx) = mpsc::channel(32);
    let tx2 = tx.clone();


    tokio::spawn(async move {
        for i in 0..10 {
            tx.send(i).await;
        }
    });


    tokio::spawn(async move {
        tx2.send(0).await;
    });

    while let Some(message) = rx.recv().await {
        println!("GOT = {}",message);
    }

}