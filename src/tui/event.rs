use std::sync::{atomic::{AtomicBool, Ordering}, Arc};
use tokio::{time::Duration, sync::broadcast};
use ratatui::crossterm::event as crossterm_event;

const EVENT_POLL_RATE: Duration = Duration::from_millis(50);
const EVENT_CHANNEL_CAPACITY: usize = 64;

#[derive(Debug, Clone)]
pub struct EventHandler {
    event_sender: broadcast::Sender<crossterm_event::Event>,
    shutdown_handle: Arc<AtomicBool>,
}

impl EventHandler {
    pub fn new() -> Self {
        let (event_sender, _) = broadcast::channel(EVENT_CHANNEL_CAPACITY);
        let shutdown_handle = Arc::new(AtomicBool::new(false));

        EventHandler {
            event_sender,
            shutdown_handle,
        }
    }

    pub fn handle_events(&self) -> tokio::task::JoinHandle<()> {
        let shutdown_signal = self.shutdown_handle.clone();
        let event_tx = self.event_sender.clone();

        tokio::spawn(async move {
            while !shutdown_signal.load(Ordering::Relaxed) {
                if crossterm_event::poll(EVENT_POLL_RATE).unwrap() {
                    if let Ok(event) = crossterm_event::read() {
                        if event_tx.send(event).is_err() {
                            println!("Failed to send event to the channel");
                            break;
                        }
                    }
                }
            }
        })
    }

    pub fn subscribe_events(&self) -> broadcast::Receiver<crossterm_event::Event> {
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
