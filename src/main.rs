use std::fs::{self, File};
use std::io::prelude::*;
use std::os::unix::fs::PermissionsExt;
use std::process::Command;
use std::time::Duration;
use tokio::time::sleep;

const RESET_IDLE_TIME_PATH: &str = "./reset_idle_time";

fn create_reset_bin() -> std::io::Result<()> {
    let bytes = include_bytes!("../bin/reset_idle_time");

    let mut file = File::create(RESET_IDLE_TIME_PATH)?;
    file.write_all(bytes)?;
    fs::set_permissions(RESET_IDLE_TIME_PATH, fs::Permissions::from_mode(0o777))?;

    Ok(())
}

fn reset_inactivity() {
    Command::new(RESET_IDLE_TIME_PATH)
        .output()
        .expect("failed to execute process");
}

async fn setup() {
    create_reset_bin().unwrap();

    loop {
        reset_inactivity();
        println!("Inactivity reset");
        sleep(Duration::from_secs(65)).await;
    }
}

#[tokio::main]
async fn main() {
    setup().await;
}
