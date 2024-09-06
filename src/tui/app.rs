use std::sync::mpsc;
use ratatui::{crossterm::event::Event, layout::Rect, Frame};

use super::widgets::common::Widget;
use super::screens::welcome;

pub enum AppCommand {
    SwitchScreen(Box<dyn Widget + Send>),
    Quit,
}

pub struct App {
    current_screen: Box<dyn Widget + Send>,
    command_rx: mpsc::Receiver<AppCommand>,
    running: bool,
}

impl App {
    pub fn new() -> anyhow::Result<Self> {
        let (command_tx, command_rx) = mpsc::channel();
        let current_screen = Box::new(welcome::WelcomeScreen::new(command_tx.clone()));
        Ok(Self { current_screen, command_rx, running: true })
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn check_app_commands(&mut self) -> anyhow::Result<()> {
        if let Ok(command) = self.command_rx.try_recv() {
            match command {
                AppCommand::SwitchScreen(screen) => {
                    self.current_screen = screen;
                },
                AppCommand::Quit => {
                    self.running = false;
                },
            }
        }
        Ok(())
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