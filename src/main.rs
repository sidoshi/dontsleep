use ctrlc;
use std::sync::mpsc;

mod caffeinate;

#[tokio::main]
async fn main() {
    let (tx, rx) = mpsc::channel::<()>();

    ctrlc::set_handler(move || {
        tx.send(()).expect("Error while sending interrupt signal");
    })
    .expect("Error setting up Signal handler");

    caffeinate::setup(&rx);
    slacker::setup(&rx).await;
}
