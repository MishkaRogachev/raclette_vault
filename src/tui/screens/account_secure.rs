
use std::sync::mpsc;
use zeroize::Zeroizing;
use ratatui::{
    crossterm::event::Event,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    widgets::Paragraph, Frame
};

use crate::{core::seed_phrase, service::session};
use crate::tui::{app::{AppCommand, AppScreen}, widgets::controls::{self, Focusable}};

const MAX_SECURE_WIDTH: u16 = 80;
const INTRO_HEIGHT: u16 = 1;
const INPUT_LABEL_HEIGHT: u16 = 1;
const TIP_HEIGHT: u16 = 1;

const INTRO_TEXT: &str = "Your master account keypair was created. Now let's secure it!";
const FIRST_LABEL_TEXT: &str = "Enter password. It will not be stored anywhere.";
const SECOND_LABEL_TEXT: &str = "Please, confirm your password.";
const TIP_TEXT: &str = "Tip: Use [Tab] to focus next input and [Esc] to reset focus.";

pub struct Screen {
    command_tx: mpsc::Sender<AppCommand>,
    seed_phrase: seed_phrase::SeedPhrase,

    first_input: controls::Input,
    second_input: controls::Input,
    back_button: controls::Button,
    reveal_button: controls::SwapButton,
    save_button: controls::Button,
}

impl Screen {
    pub fn new(command_tx: mpsc::Sender<AppCommand>, seed_phrase: seed_phrase::SeedPhrase) -> Self {
        let mut first_input = controls::Input::new("Enter password").masked();
        let second_input = controls::Input::new("Confirm password").masked();
        let back_button = controls::Button::new("Back", Some('b')).escape();
        let reveal_button = controls::SwapButton::new(
            controls::Button::new("Reveal", Some('r')).warning(),
            controls::Button::new("Hide", Some('h')).primary());
        let save_button = controls::Button::new("Save", Some('s')).default().disable();

        first_input.set_focused(true);

        Self { command_tx, seed_phrase, first_input, second_input, back_button, reveal_button, save_button }
    }
}

#[async_trait::async_trait]
impl AppScreen for Screen {
    async fn handle_event(&mut self, event: Event) -> anyhow::Result<bool> {
        let scoped_event = controls::handle_scoped_event(
            &mut [&mut self.first_input, &mut self.second_input], &event);

        let first_password = &self.first_input.value;
        let second_password = &self.second_input.value;

        let secure_action = || {
            if first_password.is_empty() || first_password != second_password { // TODO: validate password here
                return;
            }
            let password = Zeroizing::new(first_password.to_string());
            let session = session::Session::create_account(&self.seed_phrase, &password).expect("Fatal issue with creating an account");
            let porfolio = Box::new(super::porfolio::Screen::new(self.command_tx.clone(), session));
            self.command_tx.send(AppCommand::SwitchScreen(porfolio)).unwrap();
        };

        if let Some(event) = scoped_event {
            if let controls::InputEvent::FocusFinished = event {
                secure_action();
                return Ok(true);
            }
        } else {
            if let Some(()) = self.back_button.handle_event(&event) {
                let create_screeen = Box::new(super::account_create::Screen::new(
                    self.command_tx.clone(), self.seed_phrase.clone()));
                self.command_tx
                    .send(AppCommand::SwitchScreen(create_screeen))
                    .unwrap();
                return Ok(true);
            }

            if let Some(reveal) = self.reveal_button.handle_event(&event) {
                self.first_input.masked = !reveal;
                self.second_input.masked = !reveal;
            }

            if let Some(()) = self.save_button.handle_event(&event) {
                secure_action();
                return Ok(true);
            }
        }

        Ok(false)
    }

    async fn update(&mut self) {
        let first_password = &self.first_input.value;
        let second_password = &self.second_input.value;

        if first_password.is_empty() {
            self.first_input.color = Color::Red;
            self.second_input.color = Color::Red;
            self.save_button.disabled = true;
        } else {
            self.first_input.color = Color::Yellow;

            if *first_password != *second_password {
                self.second_input.color = Color::Red;
                self.save_button.disabled = true;
            } else {
                self.second_input.color = Color::Yellow;
                self.save_button.disabled = false;
            }
        }
    }

    fn render(&mut self, frame: &mut Frame, area: Rect) {
        let updated_width = area.width.min(MAX_SECURE_WIDTH);
        let centered_area = Rect { x: area.x + (area.width - updated_width) / 2, width: updated_width, ..area };

        let content_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(0), // Fill height
                Constraint::Min(INTRO_HEIGHT),
                Constraint::Length(INPUT_LABEL_HEIGHT),
                Constraint::Length(controls::INPUT_HEIGHT),
                Constraint::Min(0), // Fill height
                Constraint::Length(INPUT_LABEL_HEIGHT),
                Constraint::Length(controls::INPUT_HEIGHT),
                Constraint::Min(0), // Fill height
                Constraint::Length(TIP_HEIGHT),
                Constraint::Min(0), // Fill height
                Constraint::Length(controls::BUTTON_HEIGHT),
            ])
            .split(centered_area);

        let intro_text = Paragraph::new(INTRO_TEXT)
            .style(Style::default().fg(Color::Yellow).bold())
            .alignment(Alignment::Center);
        frame.render_widget(intro_text, content_layout[1]);

        let first_label = Paragraph::new(FIRST_LABEL_TEXT)
            .style(Style::default().fg(Color::Yellow).bold())
            .alignment(Alignment::Center);
        frame.render_widget(first_label, content_layout[2]);

        self.first_input.render(frame, content_layout[3]);

        let second_label = Paragraph::new(SECOND_LABEL_TEXT)
            .style(Style::default().fg(Color::Yellow).bold())
            .alignment(Alignment::Center);
        frame.render_widget(second_label, content_layout[5]);

        self.second_input.render(frame, content_layout[6]);

        let tip_text = Paragraph::new(TIP_TEXT)
            .style(Style::default().fg(Color::Yellow).bold())
            .alignment(Alignment::Center);
        frame.render_widget(tip_text, content_layout[8]);

        let buttons_row = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(30),
            Constraint::Percentage(30),
            Constraint::Percentage(40),
        ])
        .split(content_layout[10]);

        self.back_button.render(frame, buttons_row[0]);
        self.reveal_button.render(frame, buttons_row[1]);
        self.save_button.render(frame, buttons_row[2]);
    }
}
