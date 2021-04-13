use mouse_rs::Mouse;
use std::time::Duration;
use tokio::time::sleep;

const POINT_A: i32 = 50;
const POINT_B: i32 = 350;

fn get_position(mouse: &Mouse) -> (i32, i32) {
    let point = mouse
        .get_position()
        .expect("Error getting a mouse position");

    (point.x as i32, point.y as i32)
}

fn mouse_is_on_point_a(mouse: &Mouse) -> bool {
    let position = get_position(&mouse);
    position == (POINT_A, POINT_A)
}

fn move_to_point_a(mouse: &Mouse) {
    println!("Moving the mouse to point A");
    mouse
        .move_to(POINT_A, POINT_A)
        .expect("Error moving mouse to Point A");
}

fn move_to_point_b(mouse: &Mouse) {
    println!("Moving the mouse to point B");
    mouse
        .move_to(POINT_B, POINT_B)
        .expect("Error moving mouse to Point B");
}

async fn setup() {
    let mouse = Mouse::new();
    loop {
        sleep(Duration::from_secs(3)).await;

        if mouse_is_on_point_a(&mouse) {
            move_to_point_b(&mouse);
        } else {
            move_to_point_a(&mouse);
        }
    }
}

#[tokio::main]
async fn main() {
    setup().await;
}
