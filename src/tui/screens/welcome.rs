
use std::sync::mpsc;
use web3::types::H160;
use ratatui::{
    crossterm::event::Event,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    widgets::Paragraph, Frame
};

use crate::{core::seed_phrase::SeedPhrase, service::account::Account};
use crate::tui::{widgets::{buttons, ascii}, app::{AppCommand, AppScreen}};

const WELCOME_WIDTH: u16 = 60;
const LOGO_HEIGHT: u16 = 20;
const WARNING_HEIGHT: u16 = 1;
const BUTTONS_ROW_HEIGHT: u16 = 3;

pub struct Screen {
    command_tx: mpsc::Sender<AppCommand>,

    quit_button: buttons::Button,
    login_button: Option<(buttons::Button, H160)>,
    create_button: buttons::Button,
}

impl Screen {
    pub fn new(command_tx: mpsc::Sender<AppCommand>) -> Self {
        let quit_button = buttons::Button::new("Quit", Some('q'));
        let login_button = {
            let accounts = Account::list_accounts().expect("Failed to list accounts");
            match accounts.len() {
                0 => None,
                1 => {
                    let account = accounts.first().unwrap().clone();
                    Some((buttons::Button::new("Login", Some('l')), account))
                },
                _ => panic!("Multiple accounts are not supported yet")
            }
        };
        let create_button = buttons::Button::new("Create Master Account", Some('c'));

        Self { command_tx, quit_button, login_button, create_button }
    }
}

impl AppScreen for Screen {
    fn handle_event(&mut self, event: Event) -> anyhow::Result<()> {
        if let Some(()) = self.quit_button.handle_event(&event) {
            self.command_tx.send(AppCommand::Quit).unwrap();
            return Ok(());
        }

        if let Some((login_button, account)) = &mut self.login_button {
            if let Some(()) = login_button.handle_event(&event) {
                let login_screen = Box::new(super::login::Screen::new(self.command_tx.clone(), account.clone()));
                self.command_tx.send(AppCommand::SwitchScreen(login_screen))
                    .map_err(|e| anyhow::anyhow!(format!("Failed to send command: {}", e)))?;
                return Ok(());
            }
        }

        if let Some(()) = self.create_button.handle_event(&event) {
            let seed_phrase = SeedPhrase::generate(bip39::MnemonicType::Words12);
            let create_screen = Box::new(super::create::Screen::new(
                self.command_tx.clone(), seed_phrase));
            self.command_tx.send(AppCommand::SwitchScreen(create_screen)).unwrap();
            return Ok(());
        }
        Ok(())
    }

    fn render(&mut self, frame: &mut Frame) {
        let area = frame.area();
        let horizontal_padding = (area.width.saturating_sub(WELCOME_WIDTH)) / 2;

        let centered_area = Rect {
            x: horizontal_padding,
            y: area.y,
            width: WELCOME_WIDTH,
            height: area.height,
        };

        let content_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(0), // Fill height
                Constraint::Length(LOGO_HEIGHT),
                Constraint::Length(WARNING_HEIGHT),
                Constraint::Length(BUTTONS_ROW_HEIGHT),
                Constraint::Min(0), // Fill height
            ])
            .split(centered_area);

        let logo = Paragraph::new(ascii::BIG_LOGO)
            .style(Style::default().fg(Color::Yellow))
            .alignment(Alignment::Center);
        frame.render_widget(logo, content_layout[1]);

        let warning_text = Paragraph::new("Please don't use this wallet for real crypto!")
            .style(Style::default().fg(Color::Red).bold())
            .alignment(Alignment::Center);
        frame.render_widget(warning_text, content_layout[2]);

        let buttons_row = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(30),
                Constraint::Percentage(70),
            ])
            .split(content_layout[3]);

        self.quit_button.render(frame, buttons_row[0]);

        if let Some((login_button, _)) = &mut self.login_button {
            login_button.render(frame, buttons_row[1]);
        } else {
            self.create_button.render(frame, buttons_row[1]);
        }
    }
}
