use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc};
use ratatui::{crossterm::event::Event, layout::Rect, Frame};

use super::widgets::common::Widget;
use super::screens::welcome;

pub enum AppCommand {
    SwitchScreen(Box<dyn Widget + Send>),
    Quit,
}

pub struct App {
    shutdown_handle: Arc<AtomicBool>,
    current_screen: Box<dyn Widget + Send>,
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
            self.handle_event(event);
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

impl super::widgets::common::Widget for App {
    fn handle_event(&mut self, event: Event) -> Option<Event> {
        self.current_screen.handle_event(event)
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) {
        self.current_screen.draw(frame, area);
    }
}
