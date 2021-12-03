use core_graphics::{
    base::CGFloat,
    event::{CGEvent, CGEventTapLocation, CGEventType, CGMouseButton},
    event_source::{CGEventSource, CGEventSourceStateID},
    geometry::CGPoint,
};
use std::time::Duration;
use tokio::{sync::broadcast, time::sleep};

const MOVEMENT: CGFloat = 1.0;

fn get_current_location() -> CGPoint {
    CGEvent::new(CGEventSource::new(CGEventSourceStateID::CombinedSessionState).unwrap())
        .unwrap()
        .location()
}

fn mouse_move(point: CGPoint) {
    CGEvent::new_mouse_event(
        CGEventSource::new(CGEventSourceStateID::CombinedSessionState).unwrap(),
        CGEventType::MouseMoved,
        point,
        CGMouseButton::Left,
    )
    .unwrap()
    .post(CGEventTapLocation::HID);
}

/// Resets timer used the check if the system is idle or active.
/// System is kept active by slightly moving the mouse periodically.
/// To check if this is working,
/// install Quarts for python and run the script test.py
pub async fn setup(rx: &mut broadcast::Receiver<()>) {
    let five_minutes = Duration::from_secs(60 * 5);

    loop {
        for i in 1..=2 {
            let location = get_current_location();
            if i % 2 == 0 {
                let point = CGPoint::new(location.x + MOVEMENT, location.y + MOVEMENT);
                mouse_move(point)
            } else {
                let point = CGPoint::new(location.x - MOVEMENT, location.y - MOVEMENT);
                mouse_move(point)
            }

            println!("Keeping the system active");

            tokio::select! {
                _ = rx.recv() => {
                    println!("Killng slacker");
                    return;
                }
                _ = sleep(five_minutes) =>  {
                    continue;
                }
            };
        }
    }
}
