use std::sync::mpsc;
use ratatui::{
    crossterm::event::Event,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    widgets::Paragraph, Frame
};

use crate::{core::seed_phrase::SeedPhrase, service::session::Session};
use crate::tui::{widgets::{focus::{self, Focusable}, buttons, inputs}, app::{AppCommand, AppScreen}};

const INTRO_HEIGHT: u16 = 1;
const INPUT_LABEL_HEIGHT: u16 = 1;
const INPUT_HEIGHT: u16 = 3;
const ERROR_HEIGHT: u16 = 1;
const BUTTONS_ROW_HEIGHT: u16 = 3;

const INTRO_TEXT: &str = "Confirm removal of the seed phrase";
const LABEL_TEXT: &str = "Enter word";
const ERROR_TEXT: &str = "Incorrect word";

pub struct Screen {
    command_tx: mpsc::Sender<AppCommand>,
    session: Session,
    seed_phrase: SeedPhrase,
    word_index: usize,

    input: inputs::Input,
    back_button: buttons::Button,
    reveal_button: buttons::SwapButton,
    delete_button: buttons::Button,
}

impl Screen {
    pub fn new(command_tx: mpsc::Sender<AppCommand>, session: Session, seed_phrase: SeedPhrase) -> Self {
        let word_index = rand::random::<usize>() % seed_phrase.get_words().len();

        let mut input = inputs::Input::new("Enter word").masked();
        let back_button = buttons::Button::new("Back", Some('b'));
        let reveal_button = buttons::SwapButton::new(
            buttons::Button::new("Reveal", Some('r')).warning(),
            buttons::Button::new("Hide", Some('h')).primary());
        let delete_button = buttons::Button::new("Delete", Some('d')).warning().disable();

        input.set_focused(true);
        input.color = Color::Red;

        Self {
            command_tx,
            session,
            seed_phrase,
            word_index,
            input,
            back_button,
            reveal_button,
            delete_button
        }
    }
}

#[async_trait::async_trait]
impl AppScreen for Screen {
    fn handle_event(&mut self, event: Event) -> anyhow::Result<bool> {
        let scoped_event = focus::handle_scoped_event(&mut [&mut self.input], &event);

        let delete_action = || {
            self.session.delete_seed_phrase().expect("Failed to delete seed phrase");
            self.command_tx.send(AppCommand::SwitchScreen(Box::new(
                super::porfolio::Screen::new(self.command_tx.clone(), self.session.clone())
            ))).unwrap();
        };
        if let Some(event) = scoped_event {
            match event {
                focus::FocusableEvent::FocusFinished => {
                    delete_action();
                },
                focus::FocusableEvent::Input(word) => {
                    if let Some(phrase_word) = self.seed_phrase.get_words().get(self.word_index) {
                        let valid = word == *phrase_word;
                        self.delete_button.disabled = !valid;
                        self.input.color = if valid { Color::Yellow } else { Color::Red };
                    } else {
                        self.delete_button.disabled = true;
                        self.input.color = Color::Red;
                    };
                }
                _ => {}
            }

            return Ok(true);
        }

        if let Some(()) = self.back_button.handle_event(&event) {
            self.command_tx.send(AppCommand::SwitchScreen(Box::new(
                super::mnemonic_access::Screen::new(self.command_tx.clone(), self.session.clone())
            ))).unwrap();
            return Ok(true);
        }

        if let Some(reveal) = self.reveal_button.handle_event(&event) {
            self.input.masked = !reveal;
            return Ok(true);
        }

        if let Some(()) = self.delete_button.handle_event(&event) {
            delete_action();
            return Ok(true);
        }

        Ok(false)
    }

    async fn update(&mut self) {}

    fn render(&mut self, frame: &mut Frame, area: Rect) {
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
            .split(area);

        let intro_text = Paragraph::new(INTRO_TEXT)
            .style(Style::default().fg(Color::Yellow).bold())
            .alignment(Alignment::Center);
        frame.render_widget(intro_text, content_layout[1]);

        let label = Paragraph::new(format!("{} #{}", LABEL_TEXT, self.word_index + 1))
            .style(Style::default().fg(Color::Yellow).bold())
            .alignment(Alignment::Center);
        frame.render_widget(label, content_layout[3]);

        self.input.render(frame, content_layout[4]);

        if self.delete_button.disabled && !self.input.value.is_empty() {
            let error_text = Paragraph::new(ERROR_TEXT)
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
        self.delete_button.render(frame, buttons_row[2]);
    }
}
