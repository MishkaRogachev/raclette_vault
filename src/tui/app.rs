use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc};
use ratatui::{crossterm::event::Event, Frame};

use super::screens::welcome;

pub trait AppScreen {
    fn handle_event(&mut self, event: Event) -> anyhow::Result<()>;
    fn update(&mut self);
    fn render(&mut self, frame: &mut Frame);
}

pub enum AppCommand {
    SwitchScreen(Box<dyn AppScreen + Send>),
    Quit,
}

pub struct App {
    shutdown_handle: Arc<AtomicBool>,
    current_screen: Box<dyn AppScreen + Send>,
    command_rx: mpsc::Receiver<AppCommand>,
    events: tokio::sync::broadcast::Receiver<Event>
}

impl App {
    pub fn new(shutdown_handle: Arc<AtomicBool>, events: tokio::sync::broadcast::Receiver<Event>) -> anyhow::Result<Self> {
        let (command_tx, command_rx) = mpsc::channel();
        let current_screen = Box::new(welcome::Screen::new(command_tx.clone()));
        Ok(Self { shutdown_handle, current_screen, command_rx, events })
    }

    pub fn process_events(&mut self) {
        if let Ok(event) = self.events.try_recv() {
            self.handle_event(event).expect("Failed to handle screen event");
        }

        if let Ok(command) = self.command_rx.try_recv() {
            match command {
                AppCommand::SwitchScreen(screen) => {
                    self.current_screen = screen;
                },
                AppCommand::Quit => {
                    self.shutdown_handle.store(true, Ordering::Relaxed);
                },
            }
        }
    }
}

impl AppScreen for App {
    fn handle_event(&mut self, event: Event) -> anyhow::Result<()> {
        self.current_screen.handle_event(event)
    }

    fn update(&mut self) {
        self.current_screen.update()
    }

    fn render(&mut self, frame: &mut Frame) {
        self.current_screen.render(frame);
    }
}
