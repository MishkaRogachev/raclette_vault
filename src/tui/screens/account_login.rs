use std::sync::mpsc;
use ratatui::{
    crossterm::event::Event,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    widgets::Paragraph, Frame
};
use zeroize::Zeroizing;

use crate::service::session::Session;
use crate::tui::{widgets::controls::{self, Focusable}, app::{AppCommand, AppScreen}};

const MAX_LOGIN_WIDTH: u16 = 80;
const INTRO_HEIGHT: u16 = 1;
const ERROR_HEIGHT: u16 = 1;

const MAX_PASSWORD_ATTEMPTS: u8 = 3;

const INTRO_TEXT: &str = "Login into existing account. Please, enter your password.";
const INCORRECT_PASSWORD_TEXT: &str = "Incorrect password. Attempts left";

pub struct Screen {
    command_tx: mpsc::Sender<AppCommand>,
    address: web3::types::Address,
    remaining_attempts: u8,
    pass_error: Option<String>,

    input: controls::Input,
    back_button: controls::Button,
    reveal_button: controls::SwapButton,
    login_button: controls::Button,
}

impl Screen {
    pub fn new(command_tx: mpsc::Sender<AppCommand>, address: web3::types::Address) -> Self {
        let remaining_attempts = MAX_PASSWORD_ATTEMPTS;
        let pass_error = None;

        let mut input = controls::Input::new("Enter password").masked();
        let back_button = controls::Button::new("Back", Some('b')).escape();
        let reveal_button = controls::SwapButton::new(
            controls::Button::new("Reveal", Some('r')).warning(),
            controls::Button::new("Hide", Some('h')).primary());
        let login_button = controls::Button::new("Login", Some('l')).disable().default();

        input.set_focused(true);

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

#[async_trait::async_trait]
impl AppScreen for Screen {
    async fn handle_event(&mut self, event: Event) -> anyhow::Result<bool> {
        let scoped_event = controls::handle_scoped_event(&mut [&mut self.input], &event);

        let mut login_action = || {
            match Session::login(self.address, &self.input.value) {
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
        };

        if let Some(event) = scoped_event {
            match event {
                controls::FocusableEvent::Input(word) => {
                    self.login_button.disabled = word.is_empty();
                    return Ok(true);
                },
                controls::FocusableEvent::FocusFinished => {
                    login_action();
                    return Ok(true);
                },
                _ => {}
            }
        }

        if let Some(()) = self.back_button.handle_event(&event) {
            let welcome_screen = Box::new(super::welcome::Screen::new(self.command_tx.clone()));
            self.command_tx.send(AppCommand::SwitchScreen(welcome_screen)).unwrap();
            return Ok(true);
        }

        if let Some(reveal) = self.reveal_button.handle_event(&event) {
            self.input.masked = !reveal;
            return Ok(true);
        }

        if let Some(()) = self.login_button.handle_event(&event) {
            login_action();
        }
        Ok(false)
    }

    async fn update(&mut self) {}

    fn render(&mut self, frame: &mut Frame, area: Rect) {
        let updated_width = area.width.min(MAX_LOGIN_WIDTH);
        let centered_area = Rect { x: area.x + (area.width - updated_width) / 2, width: updated_width, ..area };

        let content_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(0), // Fill height
                Constraint::Length(INTRO_HEIGHT),
                Constraint::Min(0), // Fill height
                Constraint::Length(controls::INPUT_HEIGHT),
                Constraint::Min(0), // Fill height
                Constraint::Length(ERROR_HEIGHT),
                Constraint::Min(0), // Fill height
                Constraint::Length(controls::BUTTON_HEIGHT),
            ])
            .split(centered_area);

        let intro_text = Paragraph::new(INTRO_TEXT)
            .style(Style::default().fg(Color::Yellow).bold())
            .alignment(Alignment::Center);
        frame.render_widget(intro_text, content_layout[1]);

        self.input.render(frame, content_layout[3]);

        if let Some(error_string) = &self.pass_error {
            let error_text = Paragraph::new(error_string.clone())
                .style(Style::default().fg(Color::Red).bold())
                .alignment(Alignment::Center);
            frame.render_widget(error_text, content_layout[5]);
        }

        let buttons_row = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(30),
                Constraint::Percentage(30),
                Constraint::Percentage(40),
            ])
            .split(content_layout[7]);

        self.back_button.render(frame, buttons_row[0]);
        self.reveal_button.render(frame, buttons_row[1]);
        self.login_button.render(frame, buttons_row[2]);
    }
}
