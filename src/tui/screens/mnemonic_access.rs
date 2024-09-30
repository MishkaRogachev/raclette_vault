use std::sync::mpsc;
use ratatui::{
    crossterm::event::Event,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    widgets::Paragraph,
    Frame,
};

use crate::{core::seed_phrase::SeedPhrase, service::session::Session};
use crate::tui::app::{AppCommand, AppScreen};
use crate::tui::widgets::{controls, mnemonic};

const MAX_MNEM_ACCESS_WIDTH: u16 = 80;
const INTRO_HEIGHT: u16 = 2;

const INTRO_TEXT: &str = "This is your mnemonic seed phrase. Handle it with care!";

pub struct Screen {
    command_tx: mpsc::Sender<AppCommand>,
    session: Session,
    seed_phrase: SeedPhrase,

    mnemonic_words: mnemonic::MnemonicWords,
    back_button: controls::Button,
    reveal_button: controls::SwapButton,
    delete_button: controls::Button,
}

impl Screen {
    pub fn new(command_tx: mpsc::Sender<AppCommand>, session: Session) -> Self {
        let seed_phrase = session.db.get_seed_phrase().unwrap();

        let mnemonic_words = mnemonic::MnemonicWords::new(seed_phrase.get_words_zeroizing());
        let back_button = controls::Button::new("Back", Some('b'));
        let reveal_button = controls::SwapButton::new(
            controls::Button::new("Reveal", Some('r')).warning(),
            controls::Button::new("Hide", Some('h')).primary(),
        );
        let delete_button = controls::Button::new("Delete Mnemonic", Some('d')).warning();

        Self {
            command_tx,
            session,
            seed_phrase,
            mnemonic_words,
            back_button,
            reveal_button,
            delete_button,
        }
    }
}

#[async_trait::async_trait]
impl AppScreen for Screen {
    async fn handle_event(&mut self, event: Event) -> anyhow::Result<bool> {
        if let Some(()) = self.back_button.handle_event(&event) {
            let portfolio_screen = Box::new(super::porfolio::Screen::new(self.command_tx.clone(), self.session.clone()));
            self.command_tx
                .send(AppCommand::SwitchScreen(portfolio_screen))
                .unwrap();
            return Ok(true);
        }

        if let Some(reveal) = self.reveal_button.handle_event(&event) {
            self.mnemonic_words.masked = !reveal;
            return Ok(true);
        }

        if let Some(()) = self.delete_button.handle_event(&event) {
            let delete_mnemonic_screen = Box::new(super::mnemonic_delete::Screen::new(
                self.command_tx.clone(), self.session.clone(), self.seed_phrase.clone()));
            self.command_tx
                .send(AppCommand::SwitchScreen(delete_mnemonic_screen))
                .unwrap();
            return Ok(true);
        }
        Ok(false)
    }

    async fn update(&mut self) {}

    fn render(&mut self, frame: &mut Frame, area: Rect) {
        let updated_width = area.width.min(MAX_MNEM_ACCESS_WIDTH);
        let centered_area = Rect { x: area.x + (area.width - updated_width) / 2, width: updated_width, ..area };

        let content_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(0), // Fill height
                Constraint::Length(INTRO_HEIGHT),
                Constraint::Length(mnemonic::MNEMONIC_HEIGHT),
                Constraint::Length(controls::BUTTONS_HEIGHT),
            ])
            .split(centered_area);

        let intro_text = Paragraph::new(INTRO_TEXT)
            .style(Style::default().fg(Color::Yellow).bold())
            .alignment(Alignment::Center);
        frame.render_widget(intro_text, content_layout[1]);

        self.mnemonic_words.render(frame, content_layout[2]);

        let buttons_row = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(35),
                Constraint::Percentage(35),
                Constraint::Percentage(30),
            ])
            .split(content_layout[3]);

        self.back_button.render(frame, buttons_row[0]);
        self.reveal_button.render(frame, buttons_row[1]);
        self.delete_button.render(frame, buttons_row[2]);
    }
}
