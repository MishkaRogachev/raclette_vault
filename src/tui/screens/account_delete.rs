
use std::sync::mpsc;
use ratatui::{
    crossterm::event::Event,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    widgets::Paragraph, Frame
};

use crate::service::session::Session;
use crate::tui::{widgets::{controls, ascii}, app::{AppCommand, AppScreen}};

const MAX_DELETE_WIDTH: u16 = 80;
const SKULL_HEIGHT: u16 = 20;
const WARNING_HEIGHT: u16 = 1;

const WARNING_TEXT: &str = "Are you going to delete your account and root keypair!?";

pub struct Screen {
    command_tx: mpsc::Sender<AppCommand>,
    session: Session,

    cancel_button: controls::Button,
    delete_button: controls::Button,
}

impl Screen {
    pub fn new(command_tx: mpsc::Sender<AppCommand>, session: Session) -> Self {
        let cancel_button = controls::Button::new("Cancel", Some('c')).primary().escape();
        let delete_button = controls::Button::new("Delete Account", Some('d')).warning();

        Self { command_tx, session, cancel_button, delete_button }
    }
}

#[async_trait::async_trait]
impl AppScreen for Screen {
    async fn handle_event(&mut self, event: Event) -> anyhow::Result<bool> {
        if let Some(()) = self.cancel_button.handle_event(&event) {
            let porfolio = Box::new(super::porfolio::Screen::new(
                self.command_tx.clone(), self.session.clone()));
            self.command_tx.send(AppCommand::SwitchScreen(porfolio)).unwrap();
            return Ok(true);
        }

        if let Some(()) = self.delete_button.handle_event(&event) {
            self.session.delete_account().expect("Failed to delete account");
            let welcome_screen = Box::new(super::welcome::Screen::new(
                self.command_tx.clone()));
            self.command_tx.send(AppCommand::SwitchScreen(welcome_screen)).unwrap();
            return Ok(true);
        }
        Ok(false)
    }

    async fn update(&mut self) {}

    fn render(&mut self, frame: &mut Frame, area: Rect) {
        let updated_width = area.width.min(MAX_DELETE_WIDTH);
        let centered_area = Rect { x: area.x + (area.width - updated_width) / 2, width: updated_width, ..area };

        let content_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(0), // Fill height
                Constraint::Length(SKULL_HEIGHT),
                Constraint::Length(WARNING_HEIGHT),
                Constraint::Length(controls::BUTTONS_HEIGHT),
            ])
            .split(centered_area);

        let skull = Paragraph::new(ascii::SKULL)
            .style(Style::default().fg(Color::Red))
            .alignment(Alignment::Center);
        frame.render_widget(skull, content_layout[1]);

        let warning_text = Paragraph::new(WARNING_TEXT)
            .style(Style::default().fg(Color::Red).bold())
            .alignment(Alignment::Center);
        frame.render_widget(warning_text, content_layout[2]);

        let buttons_row = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(70),
                Constraint::Percentage(30),
            ])
            .split(content_layout[3]);

        self.cancel_button.render(frame, buttons_row[0]);
        self.delete_button.render(frame, buttons_row[1]);
    }
}
