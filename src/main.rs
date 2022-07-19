use tokio::sync::broadcast;

mod caffeinate;

#[tokio::main]
async fn main() {
    let (tx, rx1) = broadcast::channel::<()>(10);
    let rx2 = tx.subscribe();

    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.unwrap();
        tx.send(()).expect("Error sending terminate signal");
    });

    tokio::join!(caffeinate::setup(rx1), slacker::setup(rx2));
}
