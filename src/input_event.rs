use evdev;

#[derive(Debug, Clone, Copy)]
pub enum KeyState {
    Pressed,
    Released,
}

#[derive(Debug, Clone, Copy)]
pub struct KeyEvent {
    pub key_code: u16,
    pub state: KeyState,
}

#[derive(Debug, Clone, Copy)]
pub enum InputEvent {
    Key(KeyEvent),
}

impl InputEvent {
    pub fn from_evdev_event(evdev_event: evdev::InputEvent) -> Option<InputEvent> {
        if evdev_event.event_type() != evdev::EventType::KEY {
            return None;
        };

        let state = if evdev_event.value() == 1 {
            KeyState::Pressed
        } else {
            KeyState::Released
        };
        let key_code = evdev_event.code();

        Some(InputEvent::Key(KeyEvent { key_code, state }))
    }
}
