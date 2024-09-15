use std::sync::{atomic::AtomicBool, mpsc, Arc, Mutex};
use ratatui::{
    crossterm::event::Event,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    widgets::Paragraph, Frame
};
use zeroize::Zeroizing;

use crate::service::account::Account;
use crate::tui::{widgets::{common, focus, buttons, inputs}, app::AppCommand};

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
    pass_error: Arc<Mutex<Option<String>>>,

    input: inputs::Input,
    back_button: buttons::Button,
    reveal_button: buttons::SwapButton,
    login_button: buttons::Button,
}

impl Screen {
    pub fn new(command_tx: mpsc::Sender<AppCommand>, address: web3::types::Address) -> Self {
        let reveal_flag = Arc::new(AtomicBool::new(false));
        let remaining_attempts = Arc::new(Mutex::new(MAX_PASSWORD_ATTEMPTS));
        let pass_error = Arc::new(Mutex::new(None));
        let password = Arc::new(Mutex::new(Zeroizing::new(String::new())));

        let input = {
            let password = password.clone();
            inputs::Input::new("Enter password")
                .mask(reveal_flag.clone())
                .on_input(move |value| {
                    *password.lock().unwrap() = Zeroizing::new(value.to_string());
                })
        };

        let back_button = {
            let command_tx = command_tx.clone();
            buttons::Button::new("Back", Some('b'))
                .on_down(move || {
                    let welcome_screeen = Box::new(super::welcome::Screen::new(command_tx.clone()));
                    command_tx.send(AppCommand::SwitchScreen(welcome_screeen)).unwrap();
                })
        };

        let reveal_button = buttons::SwapButton::new(
            reveal_flag, "Reveal", Some('r'), "Hide", Some('h'));

        let login_button = {
            let password = password.lock().unwrap().clone();
            let pass_error = pass_error.clone();
            buttons::Button::new("Login", Some('l'))
                .on_down(move || {
                match Account::login(address, &password) {
                    Ok(account) => {
                        let home_screen = Box::new(super::porfolio::Screen::new(command_tx.clone(), account));
                        command_tx.send(AppCommand::SwitchScreen(home_screen)).unwrap();
                    },
                    Err(_) => {
                        let mut attempts = remaining_attempts.lock().unwrap();
                        if *attempts > 1 {
                            *attempts -= 1;
                            *pass_error.lock().unwrap() = Some(format!("{}: {}", INCORRECT_PASSWORD_TEXT, attempts));
                            // TODO: clear password input
                        } else {
                            command_tx.send(AppCommand::Quit).unwrap();
                        }
                    }
                }
            })
        };

        Self {
            pass_error,
            input,
            back_button,
            reveal_button,
            login_button
        }
    }
}

impl common::Widget for Screen {
    fn handle_event(&mut self, event: Event) -> Option<Event> {
        let event = focus::handle_event(&mut [&mut self.input], event);
        match event {
            Some(event) => {
                let mut controls: Vec<&mut dyn common::Widget> = vec![
                    &mut self.back_button,
                    &mut self.reveal_button,
                    &mut self.login_button
                ];
                controls.iter_mut().fold(Some(event), |event, button| {
                    event.and_then(|e| button.handle_event(e))
                })
            },
            None => None
        }
    }

    fn process(&mut self, frame: &mut Frame, area: Rect) {
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

        self.input.process(frame, content_layout[4]);

        if let Some(error) = self.pass_error.lock().unwrap().as_ref() {
            let error_text = Paragraph::new(error.clone())
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

        self.back_button.process(frame, buttons_row[0]);
        self.reveal_button.process(frame, buttons_row[1]);
        self.login_button.process(frame, buttons_row[2]);
    }
}