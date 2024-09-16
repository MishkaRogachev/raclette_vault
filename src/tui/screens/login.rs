use std::sync::mpsc;
use ratatui::{
    crossterm::event::Event,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    widgets::Paragraph, Frame
};
use zeroize::Zeroizing;

use crate::service::account::Account;
use crate::tui::{widgets::{focus, buttons, inputs}, app::{AppCommand, AppScreen}};

const HOME_WIDTH: u16 = 60;
const INTRO_HEIGHT: u16 = 1;
const INPUT_LABEL_HEIGHT: u16 = 1;
const INPUT_HEIGHT: u16 = 3;
const ERROR_HEIGHT: u16 = 1;
const BUTTONS_ROW_HEIGHT: u16 = 3;

const MAX_PASSWORD_ATTEMPTS: u8 = 3;

const INTRO_TEXT: &str = "Login into existing account";
const LABEL_TEXT: &str = "Enter password";
const INCORRECT_PASSWORD_TEXT: &str = "Incorrect password. Attempts left";

pub struct Screen { 
    command_tx: mpsc::Sender<AppCommand>,
    address: web3::types::Address,
    remaining_attempts: u8,
    pass_error: Option<String>,

    input: inputs::Input,
    back_button: buttons::Button,
    reveal_button: buttons::SwapButton,
    login_button: buttons::Button,
}

impl Screen {
    pub fn new(command_tx: mpsc::Sender<AppCommand>, address: web3::types::Address) -> Self {
        let remaining_attempts = MAX_PASSWORD_ATTEMPTS;
        let pass_error = None;

        let input = inputs::Input::new("Enter password").masked();
        let back_button = buttons::Button::new("Back", Some('b'));
        let reveal_button = buttons::SwapButton::new(
            buttons::Button::new("Reveal", Some('r')).warning(),
            buttons::Button::new("Hide", Some('h')).primary());
        let login_button = buttons::Button::new("Login", Some('l'));

        Self {
            command_tx,
            address,
            remaining_attempts,
            pass_error,
            input,
            back_button,
            reveal_button,
            login_button
        }
    }
}

impl AppScreen for Screen {
    fn handle_event(&mut self, event: Event) -> anyhow::Result<()> {
        focus::handle_scoped_event(&mut [&mut self.input], &event);

        if let Some(()) = self.back_button.handle_event(&event) {
            let welcome_screen = Box::new(super::welcome::Screen::new(self.command_tx.clone()));
            self.command_tx.send(AppCommand::SwitchScreen(welcome_screen)).unwrap();
            return Ok(());
        }

        if let Some(reveal) = self.reveal_button.handle_event(&event) {
            self.input.masked = !reveal;
            return Ok(());
        }

        if let Some(()) = self.login_button.handle_event(&event) {
            match Account::login(self.address, &self.input.value) {
                Ok(account) => {
                    let porfolio = Box::new(super::porfolio::Screen::new(
                        self.command_tx.clone(), account));
                    self.command_tx.send(AppCommand::SwitchScreen(porfolio)).unwrap();
                },
                Err(_) => {
                    if self.remaining_attempts > 1 {
                        self.remaining_attempts -= 1;
                        self.pass_error = Some(format!("{}: {}", INCORRECT_PASSWORD_TEXT, self.remaining_attempts));
                        self.input.value = Zeroizing::new(String::new());
                    } else {
                        self.command_tx.send(AppCommand::Quit).unwrap();
                    }
                }
            }
        }

        Ok(())
    }

    fn render(&mut self, frame: &mut Frame) {
        let area = frame.area();
        self.login_button.disabled = self.input.value.is_empty();

        let horizontal_padding = (area.width.saturating_sub(HOME_WIDTH)) / 2;

        let centered_area = Rect {
            x: horizontal_padding,
            y: area.y,
            width: HOME_WIDTH,
            height: area.height,
        };

        let content_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(0), // Fill height
                Constraint::Length(INTRO_HEIGHT),
                Constraint::Min(0), // Fill height
                Constraint::Length(INPUT_LABEL_HEIGHT),
                Constraint::Length(INPUT_HEIGHT),
                Constraint::Min(0), // Fill height
                Constraint::Length(ERROR_HEIGHT),
                Constraint::Min(0), // Fill height
                Constraint::Length(BUTTONS_ROW_HEIGHT),
                Constraint::Min(0), // Fill height
            ])
            .split(centered_area);

        let intro_text = Paragraph::new(INTRO_TEXT)
            .style(Style::default().fg(Color::Yellow).bold())
            .alignment(Alignment::Center);
        frame.render_widget(intro_text, content_layout[1]);

        let label = Paragraph::new(LABEL_TEXT)
            .style(Style::default().fg(Color::Yellow).bold())
            .alignment(Alignment::Center);
        frame.render_widget(label, content_layout[3]);

        self.input.render(frame, content_layout[4]);

        if let Some(error_string) = &self.pass_error {
            let error_text = Paragraph::new(error_string.clone())
                .style(Style::default().fg(Color::Red).bold())
                .alignment(Alignment::Center);
            frame.render_widget(error_text, content_layout[6]);
        }

        let buttons_row = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(30),
                Constraint::Percentage(30),
                Constraint::Percentage(40),
            ])
            .split(content_layout[8]);

        self.back_button.render(frame, buttons_row[0]);
        self.reveal_button.render(frame, buttons_row[1]);
        self.login_button.render(frame, buttons_row[2]);
    }
}
