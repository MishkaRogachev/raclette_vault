use std::sync::{atomic::{AtomicBool, Ordering}, Arc};
use tokio::{time::Duration, sync::broadcast};
use ratatui::crossterm::event::{poll, read, Event, KeyCode, KeyModifiers};

const EVENT_POLL_RATE: Duration = Duration::from_millis(50);
const EVENT_CHANNEL_CAPACITY: usize = 64;

#[derive(Debug, Clone)]
pub struct EventHandler {
    event_sender: broadcast::Sender<Event>,
    shutdown_handle: Arc<AtomicBool>,
}

impl EventHandler {
    pub fn new(shutdown_handle: Arc<AtomicBool>) -> Self {
        let (event_sender, _) = broadcast::channel(EVENT_CHANNEL_CAPACITY);

        EventHandler {
            event_sender,
            shutdown_handle,
        }
    }

    pub fn handle_events(self) -> tokio::task::JoinHandle<()> {
        tokio::spawn(async move {
            while !self.shutdown_handle.load(Ordering::Relaxed) {
                if poll(EVENT_POLL_RATE).unwrap() {
                    if let Ok(event) = read() {
                        if let Event::Key(key_event) = event {
                            if (key_event.code == KeyCode::Char('c') || key_event.code == KeyCode::Char('C'))
                                && key_event.modifiers == KeyModifiers::CONTROL {
                                self.shutdown_handle.store(true, Ordering::Relaxed);
                                return;
                            }
                        }

                        if self.event_sender.send(event).is_err() {
                            println!("Failed to send event to the channel");
                            break;
                        }
                    }
                }
            }
        })
    }

    pub fn subscribe_events(&self) -> broadcast::Receiver<Event> {
        self.event_sender.subscribe()
    }

    pub fn stop(&mut self) -> anyhow::Result<()> {
        self.shutdown_handle.store(true, Ordering::Relaxed);
        Ok(())
    }
}

impl Drop for EventHandler {
    fn drop(&mut self) {
        self.stop().expect("failed to stop the event handler");
    }
}
