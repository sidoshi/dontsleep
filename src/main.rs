use tokio::sync::broadcast;

mod caffeinate;

#[tokio::main]
async fn main() {
    let (tx, mut rx1) = broadcast::channel::<()>(10);
    let mut rx2 = tx.subscribe();

    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.unwrap();
        tx.send(()).expect("Error sending terminate signal");
    });

    tokio::join!(caffeinate::setup(&mut rx1), slacker::setup(&mut rx2));
}
