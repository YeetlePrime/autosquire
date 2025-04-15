use crate::input_event::InputEvent;

pub trait InputListener {
    fn new() -> Self
    where
        Self: Sized;
    fn listen(&self) -> InputEvent;
}
