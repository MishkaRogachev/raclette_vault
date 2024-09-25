use std::sync::mpsc;
use ratatui::{
    crossterm::event::Event,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    widgets::Paragraph,
    Frame,
};
use zeroize::Zeroizing;

use crate::core::seed_phrase::SeedPhrase;
use crate::tui::app::{AppCommand, AppScreen};
use crate::tui::widgets::{buttons, mnemonic};

const IMPORT_WIDTH: u16 = 80;
const INTRO_HEIGHT: u16 = 2;
const BUTTONS_ROW_HEIGHT: u16 = 3;

const VALID_SEED_PHRASE: &str = "Your seed phrase was sucesfully imported! You may access it later in the app.";
const INVALID_SEED_PHRASE: &str = "Your seed phrase is incorrect!";

pub struct Screen {
    command_tx: mpsc::Sender<AppCommand>,
    seed_phrase: Option<SeedPhrase>,

    mnemonic_words: mnemonic::MnemonicWords,
    back_button: buttons::Button,
    reveal_button: buttons::SwapButton,
    secure_button: buttons::Button,
}

impl Screen {
    pub fn new(command_tx: mpsc::Sender<AppCommand>, words: Vec<Zeroizing<String>>) -> Self {
        let seed_phrase = match SeedPhrase::from_words(words.iter().map(
            |w| w.to_string()).collect()) {
            Ok(seed_phrase) => Some(seed_phrase),
            Err(_) => None,
        };

        let mut mnemonic_words = mnemonic::MnemonicWords::new(words);
        let back_button = buttons::Button::new("Back", Some('b'));
        let reveal_button = buttons::SwapButton::new(
            buttons::Button::new("Reveal", Some('r')).warning(),
            buttons::Button::new("Hide", Some('h')).primary(),
        );
        let mut secure_button = buttons::Button::new("Secure", Some('s'));
        if seed_phrase.is_none() {
            secure_button.disabled = true;
            mnemonic_words.color = Color::Red;
        }

        Self {
            command_tx,
            seed_phrase,
            mnemonic_words,
            back_button,
            reveal_button,
            secure_button,
        }
    }
}

impl AppScreen for Screen {
    fn handle_event(&mut self, event: Event) -> anyhow::Result<()> {
        if let Some(()) = self.back_button.handle_event(&event) {
            let words = self.mnemonic_words.words.clone();
            let mtype = match words.len() {
                12 => bip39::MnemonicType::Words12,
                24 => bip39::MnemonicType::Words24,
                _ => unreachable!(),
            };
            let index = words.len() - 1;
            let import_screen = Box::new(super::account_import_words::Screen::new(
                self.command_tx.clone(), mtype, words, index, false));
            self.command_tx.send(AppCommand::SwitchScreen(import_screen)).unwrap();
            return Ok(());
        }

        if let Some(reveal) = self.reveal_button.handle_event(&event) {
            self.mnemonic_words.masked = !reveal;
            return Ok(());
        }

        if let Some(()) = self.secure_button.handle_event(&event) {
            if let Some(seed_phrase) = &self.seed_phrase {
                let secure_screeen = Box::new(super::account_secure::Screen::new(
                    self.command_tx.clone(), seed_phrase.clone()));
                self.command_tx
                    .send(AppCommand::SwitchScreen(secure_screeen))
                    .unwrap();
                return Ok(());
            }
        }
        Ok(())
    }

    fn update(&mut self) {}

    fn render(&mut self, frame: &mut Frame) {
        let area = frame.area();
        let horizontal_padding = (area.width.saturating_sub(IMPORT_WIDTH)) / 2;

        let centered_area = Rect {
            x: horizontal_padding,
            y: area.y,
            width: IMPORT_WIDTH,
            height: area.height,
        };

        let content_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(INTRO_HEIGHT),
                Constraint::Length(mnemonic::MNEMONIC_HEIGHT),
                Constraint::Length(BUTTONS_ROW_HEIGHT),
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
