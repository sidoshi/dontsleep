use std::process::Command;
use tokio::sync::broadcast;

pub async fn setup(mut rx: broadcast::Receiver<()>) {
    let mut child = Command::new("caffeinate")
        .arg("-d")
        .spawn()
        .expect("Error spawning caffeinate");

    if let Ok(_) = rx.recv().await {
        println!("Killng caffeinate");
        child.kill().expect("Error while killing caffeinate");
    }
}
