use evdev::KeyCode;
use input_event::InputEvent;
use listener::input_listener::InputListener;

mod input_event;
mod listener;

fn main() {
    let listener = listener::platforms::wayland::WaylandListener::new();

    loop {
        let event = listener.listen();

        println!("{:?}", event);
    }
}
