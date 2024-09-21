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
const SWITCH_HEIGHT: u16 = 3;
const BUTTONS_ROW_HEIGHT: u16 = 3;

const INTRO_TEXT: &str = "Your master account will be based on the mnemonic seed phrase. \nYou may access it later in the app. Handle it with care!";

pub struct Screen {
    command_tx: mpsc::Sender<AppCommand>,
    seed_phrase: SeedPhrase,

    mnemonic_words: mnemonic::MnemonicWords,
    back_button: buttons::Button,
    reveal_button: buttons::SwapButton,
    secure_button: buttons::Button,
}

impl Screen {
    pub fn new(command_tx: mpsc::Sender<AppCommand>, seed_phrase: SeedPhrase) -> Self {
        let mnemonic_words = mnemonic::MnemonicWords::new(seed_phrase.get_words());
        let back_button = buttons::Button::new("Back", Some('b'));
        let reveal_button = buttons::SwapButton::new(
            buttons::Button::new("Reveal", Some('r')).warning(),
            buttons::Button::new("Hide", Some('h')).primary(),
        );
        let secure_button = buttons::Button::new("Secure", Some('s'));

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
            let mtype = self.seed_phrase.get_mnemonic_type();
            let words: Vec<Zeroizing<String>> = self.seed_phrase.get_words().iter()
                .map(|w| Zeroizing::new(w.to_string())).collect();
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
            let secure_screeen = Box::new(super::account_secure::Screen::new(
                self.command_tx.clone(),
                self.seed_phrase.clone(),
            ));
            self.command_tx
                .send(AppCommand::SwitchScreen(secure_screeen))
                .unwrap();
            return Ok(());
        }
        Ok(())
    }

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
                Constraint::Min(0), // Fill height
                Constraint::Length(INTRO_HEIGHT),
                Constraint::Min(0), // Fill height
                Constraint::Length(mnemonic::MNEMONIC_HEIGHT),
                Constraint::Length(BUTTONS_ROW_HEIGHT),
                Constraint::Min(0), // Fill height
            ])
            .split(centered_area);

        let intro_text = Paragraph::new(INTRO_TEXT)
            .style(Style::default().fg(Color::Yellow).bold())
            .alignment(Alignment::Center);
        frame.render_widget(intro_text, content_layout[1]);

        self.mnemonic_words.render(frame, content_layout[3]);

        let buttons_row = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(30),
                Constraint::Percentage(30),
                Constraint::Percentage(40),
            ])
            .split(content_layout[4]);

        self.back_button.render(frame, buttons_row[0]);
        self.reveal_button.render(frame, buttons_row[1]);
        self.secure_button.render(frame, buttons_row[2]);
    }
}
