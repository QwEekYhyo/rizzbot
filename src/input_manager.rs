use enigo::{Button, Coordinate::Abs, Direction::{Press, Release}, Enigo, Keyboard, Mouse, Settings};

use std::thread;
use std::time::Duration;

pub fn send_snap<'a, I>(points: I, user: &str)
where
    I: IntoIterator<Item = &'a (i32, i32)>,
{
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    let wait_time = Duration::from_millis(100);

    let mut clicks_number = 0;
    for point in points {
        enigo.move_mouse(point.0, point.1, Abs).unwrap();
        thread::sleep(wait_time);

        enigo.button(Button::Left, Press).unwrap();
        enigo.button(Button::Left, Release).unwrap();
        thread::sleep(wait_time);
        clicks_number += 1;

        if clicks_number == 2 {
            enigo.text(user).unwrap();
        }
    }
}
