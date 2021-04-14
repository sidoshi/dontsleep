use mouse_rs::Mouse;
use std::time::Duration;
use tokio::time::sleep;

const MOVEMENT: i32 = 1;

fn get_position(mouse: &Mouse) -> (i32, i32) {
    let point = mouse
        .get_position()
        .expect("Error getting a mouse position");

    (point.x as i32, point.y as i32)
}

fn move_left(mouse: &Mouse) {
    let (x, y) = get_position(&mouse);
    let x = x - MOVEMENT;
    println!("Moving the mouse to the left");
    mouse.move_to(x, y).expect("Error moving mouse to the left");
}

fn move_right(mouse: &Mouse) {
    let (x, y) = get_position(&mouse);
    let x = x + MOVEMENT;
    println!("Moving the mouse to the right");
    mouse
        .move_to(x, y)
        .expect("Error moving mouse to the right");
}

async fn setup() {
    let mouse = Mouse::new();
    loop {
        for i in 1..=10 {
            if i % 2 == 0 {
                move_left(&mouse);
            } else {
                move_right(&mouse);
            }
            sleep(Duration::from_secs(60 * 10)).await;
        }
    }
}

#[tokio::main]
async fn main() {
    setup().await;
}
