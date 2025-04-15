use std::{
    collections::VecDeque,
    sync::{Arc, Condvar, Mutex, atomic::AtomicBool},
    thread,
    time::Duration,
};

use crate::{input_event::InputEvent, listener::input_listener::InputListener};
use evdev as ed;

type EventBuffer = Arc<(Mutex<VecDeque<InputEvent>>, Condvar)>;

pub struct WaylandListener {
    // notify the listen function that a new event occured
    should_kill_threads: Arc<AtomicBool>,
    event_buffer: EventBuffer,
    thread_handles: Vec<thread::JoinHandle<()>>,
}

impl InputListener for WaylandListener {
    fn new() -> WaylandListener {
        let should_kill_threads = Arc::new(AtomicBool::new(false));

        let event_buffer = Arc::new((Mutex::new(VecDeque::new()), Condvar::new()));

        let thread_handles = ed::enumerate()
            .map(|(_path, device)| device)
            .map(|device| {
                spawn_listener_thread(
                    device,
                    Arc::clone(&event_buffer),
                    Arc::clone(&should_kill_threads),
                )
            })
            .collect();

        WaylandListener {
            should_kill_threads,
            event_buffer,
            thread_handles,
        }
    }

    fn listen(&self) -> InputEvent {
        let (event_buffer_lock, event_occured_cond) = &*self.event_buffer;

        let mut event_buffer_guard = event_buffer_lock.lock().unwrap();

        while event_buffer_guard.is_empty() {
            event_buffer_guard = event_occured_cond.wait(event_buffer_guard).unwrap();
        }

        event_buffer_guard
            .pop_front()
            .expect("The while loop should make sure that this value exists.")
    }
}

static SLEEP_INTERVAL: Duration = Duration::from_millis(10);

fn spawn_listener_thread(
    mut device: ed::Device,
    event_buffer: EventBuffer,
    should_kill_thread: Arc<AtomicBool>,
) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        println!("Spawning thread for {}", device.name().unwrap_or("Unknown"));
        let _ = device.set_nonblocking(true);
        let (event_buffer_lock, event_occured_cond) = &*event_buffer;

        while !should_kill_thread.load(std::sync::atomic::Ordering::Relaxed) {
            if let Ok(events) = device.fetch_events() {
                for event in events.filter_map(InputEvent::from_evdev_event) {
                    let mut event_buffer_guard = event_buffer_lock.lock().unwrap();
                    event_buffer_guard.push_back(event);
                    event_occured_cond.notify_all();
                }
                thread::sleep(SLEEP_INTERVAL);
            }
        }
    })
}
