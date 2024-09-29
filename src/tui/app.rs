use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc};
use ratatui::{
    crossterm::event::Event,
    layout::Rect,
    Frame
};

use super::screens::welcome;

pub const MIN_APP_WIDTH: u16 = 60;
pub const MAX_APP_WIDTH: u16 = 120;

#[async_trait::async_trait]
pub trait AppScreen {
    async fn handle_event(&mut self, event: Event) -> anyhow::Result<bool>;
    async fn update(&mut self);
    fn render(&mut self, frame: &mut Frame, area: Rect);
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

    pub async fn process_events(&mut self) {
        if let Ok(event) = self.events.try_recv() {
            self.handle_event(event).await.expect("Failed to handle screen event");
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

#[async_trait::async_trait]
impl AppScreen for App {
    async fn handle_event(&mut self, event: Event) -> anyhow::Result<bool> {
        self.current_screen.handle_event(event).await
    }

    async fn update(&mut self) {
        self.current_screen.update().await
    }

    fn render(&mut self, frame: &mut Frame, area: Rect) {
        let app_width = area.width.clamp(MIN_APP_WIDTH, MAX_APP_WIDTH);
        let horizontal_padding = (area.width.saturating_sub(app_width)) / 2;

        let app_area = Rect {
            x: horizontal_padding,
            y: area.y,
            width: app_width,
            height: area.height,
        };

        self.current_screen.render(frame, app_area);
    }
}
