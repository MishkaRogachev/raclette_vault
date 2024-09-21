
use std::sync::mpsc;
use ratatui::{
    crossterm::event::Event,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    widgets::Paragraph, Frame
};

use crate::service::account::Account;
use crate::tui::{widgets::{buttons, ascii}, app::{AppCommand, AppScreen}};

const DELETE_ACCOUNT_WIDTH: u16 = 60;
const SKULL_HEIGHT: u16 = 20;
const WARNING_HEIGHT: u16 = 1;
const BUTTONS_ROW_HEIGHT: u16 = 3;

const WARNING_TEXT: &str = "Are you going to delete your account and root keypair!?";

pub struct Screen {
    command_tx: mpsc::Sender<AppCommand>,
    account: Account,

    cancel_button: buttons::Button,
    delete_button: buttons::Button,
}

impl Screen {
    pub fn new(command_tx: mpsc::Sender<AppCommand>, account: Account) -> Self {
        let cancel_button = buttons::Button::new("Cancel", Some('c')).primary();
        let delete_button = buttons::Button::new("Delete Account", Some('d')).warning();

        Self { command_tx, account, cancel_button, delete_button }
    }
}

impl AppScreen for Screen {
    fn handle_event(&mut self, event: Event) -> anyhow::Result<()> {
        if let Some(()) = self.cancel_button.handle_event(&event) {
            let porfolio = Box::new(super::porfolio::Screen::new(
                self.command_tx.clone(), self.account.clone()));
            self.command_tx.send(AppCommand::SwitchScreen(porfolio)).unwrap();
            return Ok(());
        }

        if let Some(()) = self.delete_button.handle_event(&event) {
            self.account.delete_account().expect("Failed to delete account");
            let welcome_screen = Box::new(super::welcome::Screen::new(
                self.command_tx.clone()));
            self.command_tx.send(AppCommand::SwitchScreen(welcome_screen)).unwrap();
            return Ok(());
        }
        Ok(())
    }

    fn render(&mut self, frame: &mut Frame) {
        let area = frame.area();
        let horizontal_padding = (area.width.saturating_sub(DELETE_ACCOUNT_WIDTH)) / 2;

        let centered_area = Rect {
            x: horizontal_padding,
            y: area.y,
            width: DELETE_ACCOUNT_WIDTH,
            height: area.height,
        };

        let content_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(0), // Fill height
                Constraint::Length(SKULL_HEIGHT),
                Constraint::Length(WARNING_HEIGHT),
                Constraint::Length(BUTTONS_ROW_HEIGHT),
                Constraint::Min(0), // Fill height
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
