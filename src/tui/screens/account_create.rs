use std::sync::mpsc;
use ratatui::{
    crossterm::event::Event,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    widgets::Paragraph,
    Frame,
};

use crate::core::seed_phrase::{SeedPhrase, WordCount};
use crate::tui::app::{AppCommand, AppScreen};
use crate::tui::widgets::{controls, mnemonic};

const MAX_CREATE_WIDTH: u16 = 80;
const INTRO_HEIGHT: u16 = 1;

const INTRO_TEXT: &str = "This is your mnemonic seed phrase. You may access it later in the app.";

pub struct Screen {
    command_tx: mpsc::Sender<AppCommand>,
    seed_phrase: SeedPhrase,

    word_cnt_switch: controls::MultiSwitch,
    mnemonic_words: mnemonic::MnemonicWords,
    back_button: controls::Button,
    reveal_button: controls::SwapButton,
    secure_button: controls::Button,
}

impl Screen {
    pub fn new(command_tx: mpsc::Sender<AppCommand>, seed_phrase: SeedPhrase) -> Self {
        let word_cnt_switch = controls::MultiSwitch::new(vec![
                controls::Button::new("12 words", Some('1')), controls::Button::new("24 words", Some('2'))]);
        let mnemonic_words = mnemonic::MnemonicWords::new(seed_phrase.get_words_zeroizing());
        let back_button = controls::Button::new("Back", Some('b'));
        let reveal_button = controls::SwapButton::new(
            controls::Button::new("Reveal", Some('r')).warning(),
            controls::Button::new("Hide", Some('h')).primary(),
        );
        let secure_button = controls::Button::new("Secure", Some('s'));

        Self {
            command_tx,
            seed_phrase,
            word_cnt_switch,
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
        if let Some(is_on) = self.word_cnt_switch.handle_event(&event) {
            self.seed_phrase = SeedPhrase::generate(if is_on == 1 {
                WordCount::Words24 } else { WordCount::Words12 })?;
            self.mnemonic_words.words = self.seed_phrase.get_words_zeroizing();
            return Ok(true);
        }

        if let Some(()) = self.back_button.handle_event(&event) {
            let welcome_screen = Box::new(super::welcome::Screen::new(self.command_tx.clone()));
            self.command_tx
                .send(AppCommand::SwitchScreen(welcome_screen))
                .unwrap();
            return Ok(true);
        }

        if let Some(reveal) = self.reveal_button.handle_event(&event) {
            self.mnemonic_words.masked = !reveal;
            return Ok(true);
        }

        if let Some(()) = self.secure_button.handle_event(&event) {
            let secure_screeen = Box::new(super::account_secure::Screen::new(
                self.command_tx.clone(),
                self.seed_phrase.clone(),
            ));
            self.command_tx
                .send(AppCommand::SwitchScreen(secure_screeen))
                .unwrap();
            return Ok(true);
        }
        Ok(false)
    }

    async fn update(&mut self) {}

    fn render(&mut self, frame: &mut Frame, area: Rect) {
        let updated_width = area.width.min(MAX_CREATE_WIDTH);
        let centered_area = Rect { x: area.x + (area.width - updated_width) / 2, width: updated_width, ..area };

        let content_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(INTRO_HEIGHT),
                Constraint::Length(controls::SWITCH_HEIGHT),
                Constraint::Length(mnemonic::MNEMONIC_HEIGHT),
                Constraint::Length(controls::BUTTONS_HEIGHT),
            ])
            .split(centered_area);

        let intro_text = Paragraph::new(INTRO_TEXT)
            .style(Style::default().fg(Color::Yellow).bold())
            .alignment(Alignment::Center);
        frame.render_widget(intro_text, content_layout[0]);

        self.word_cnt_switch.render(frame, content_layout[1]);
        self.mnemonic_words.render(frame, content_layout[2]);

        let buttons_row = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(30),
                Constraint::Percentage(30),
                Constraint::Percentage(40),
            ])
            .split(content_layout[3]);

        self.back_button.render(frame, buttons_row[0]);
        self.reveal_button.render(frame, buttons_row[1]);
        self.secure_button.render(frame, buttons_row[2]);
    }
}
