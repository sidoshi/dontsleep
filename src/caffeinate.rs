use std::process::Command;
use tokio::sync::broadcast;

pub async fn setup(rx: &mut broadcast::Receiver<()>) {
    let mut child = Command::new("caffeinate")
        .spawn()
        .expect("Error spawning caffeinate");

    if let Ok(_) = rx.recv().await {
        println!("Killng caffeinate");
        child.kill().expect("Error while killing caffeinate");
    }
}
