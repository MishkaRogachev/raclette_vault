use std::sync::mpsc;
use ratatui::{
    crossterm::event::Event,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    widgets::Paragraph,
    Frame,
};
use zeroize::Zeroizing;

use crate::core::seed_phrase::{WordCount, SeedPhrase};
use crate::tui::app::{AppCommand, AppScreen};
use crate::tui::widgets::{controls, mnemonic};

const MAX_IMPORT_WIDTH: u16 = 80;
const INTRO_HEIGHT: u16 = 2;

const VALID_SEED_PHRASE: &str = "Your seed phrase was sucesfully imported! You may access it later in the app.";
const INVALID_SEED_PHRASE: &str = "Your seed phrase is not correct! Please go back and fix it!";

pub struct Screen {
    command_tx: mpsc::Sender<AppCommand>,
    seed_phrase: Option<SeedPhrase>,
    word_count: WordCount,

    mnemonic_words: mnemonic::MnemonicWords,
    back_button: controls::Button,
    reveal_button: controls::SwapButton,
    secure_button: controls::Button,
}

impl Screen {
    pub fn new(command_tx: mpsc::Sender<AppCommand>, words: Vec<Zeroizing<String>>, word_count: WordCount) -> Self {
        let seed_phrase = match SeedPhrase::from_words(words.iter().map(
            |w| w.to_string()).collect()) {
            Ok(seed_phrase) => Some(seed_phrase),
            Err(_) => None,
        };

        let mut mnemonic_words = mnemonic::MnemonicWords::new(words);
        let back_button = controls::Button::new("Back", Some('b')).escape();
        let reveal_button = controls::SwapButton::new(
            controls::Button::new("Reveal", Some('r')).warning(),
            controls::Button::new("Hide", Some('h')).primary(),
        );
        let mut secure_button = controls::Button::new("Secure", Some('s')).default();
        if seed_phrase.is_none() {
            secure_button.disabled = true;
            mnemonic_words.color = Color::Red;
        }

        Self {
            command_tx,
            seed_phrase,
            word_count,
            mnemonic_words,
            back_button,
            reveal_button,
            secure_button,
        }
    }
}

#[async_trait::async_trait]
impl AppScreen for Screen {
    async fn handle_event(&mut self, event: Event) -> anyhow::Result<bool> {
        if let Some(()) = self.back_button.handle_event(&event) {
            let words = self.mnemonic_words.words.clone();
            let index = words.len() - 1;
            let import_screen = Box::new(super::account_import_words::Screen::new(
                self.command_tx.clone(), self.word_count, words, index, false));
            self.command_tx.send(AppCommand::SwitchScreen(import_screen)).unwrap();
            return Ok(true);
        }

        if let Some(reveal) = self.reveal_button.handle_event(&event) {
            self.mnemonic_words.masked = !reveal;
            return Ok(true);
        }

        if let Some(()) = self.secure_button.handle_event(&event) {
            if let Some(seed_phrase) = &self.seed_phrase {
                let secure_screeen = Box::new(super::account_secure::Screen::new(
                    self.command_tx.clone(), seed_phrase.clone()));
                self.command_tx
                    .send(AppCommand::SwitchScreen(secure_screeen))
                    .unwrap();
                return Ok(true);
            }
        }
        Ok(false)
    }

    async fn update(&mut self) {}

    fn render(&mut self, frame: &mut Frame, area: Rect) {
        let updated_width = area.width.min(MAX_IMPORT_WIDTH);
        let centered_area = Rect { x: area.x + (area.width - updated_width) / 2, width: updated_width, ..area };

        let content_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(INTRO_HEIGHT),
                Constraint::Length(mnemonic::MNEMONIC_HEIGHT),
                Constraint::Length(controls::BUTTONS_HEIGHT),
            ])
            .split(centered_area);

            if self.seed_phrase.is_some() {
                let outro_text = Paragraph::new(VALID_SEED_PHRASE)
                    .style(Style::default().fg(Color::Yellow).bold())
                    .alignment(Alignment::Center);
                frame.render_widget(outro_text, content_layout[0]);
            } else {
                let outro_text = Paragraph::new(INVALID_SEED_PHRASE)
                    .style(Style::default().fg(Color::Red).bold())
                    .alignment(Alignment::Center);
                frame.render_widget(outro_text, content_layout[0]);
            }

        self.mnemonic_words.render(frame, content_layout[1]);

        let buttons_row = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(30),
                Constraint::Percentage(30),
                Constraint::Percentage(40),
            ])
            .split(content_layout[2]);

        self.back_button.render(frame, buttons_row[0]);
        self.reveal_button.render(frame, buttons_row[1]);
        self.secure_button.render(frame, buttons_row[2]);
    }
}
