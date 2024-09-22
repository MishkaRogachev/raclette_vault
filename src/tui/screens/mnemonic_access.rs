use std::sync::mpsc;
use ratatui::{
    crossterm::event::Event,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    widgets::Paragraph,
    Frame,
};
use zeroize::Zeroizing;

use crate::{core::seed_phrase::SeedPhrase, service::account::Account};
use crate::tui::app::{AppCommand, AppScreen};
use crate::tui::widgets::{buttons, mnemonic};

const ACCESS_MNEMONIC_WIDTH: u16 = 80;
const INTRO_HEIGHT: u16 = 2;
const BUTTONS_ROW_HEIGHT: u16 = 3;

const INTRO_TEXT: &str = "This is your mnemonic seed phrase. Handle it with care!";

pub struct Screen {
    command_tx: mpsc::Sender<AppCommand>,
    account: Account,
    seed_phrase: SeedPhrase,

    mnemonic_words: mnemonic::MnemonicWords,
    back_button: buttons::Button,
    reveal_button: buttons::SwapButton,
    delete_button: buttons::Button,
}

impl Screen {
    pub fn new(command_tx: mpsc::Sender<AppCommand>, account: Account) -> Self {
        let seed_phrase = account.get_seed_phrase().unwrap();

        let mnemonic_words = mnemonic::MnemonicWords::new(seed_phrase.get_words_zeroizing());
        let back_button = buttons::Button::new("Back", Some('b'));
        let reveal_button = buttons::SwapButton::new(
            buttons::Button::new("Reveal", Some('r')).warning(),
            buttons::Button::new("Hide", Some('h')).primary(),
        );
        let delete_button = buttons::Button::new("Delete Mnemonic", Some('d')).warning();

        Self {
            command_tx,
            account,
            seed_phrase,
            mnemonic_words,
            back_button,
            reveal_button,
            delete_button,
        }
    }
}

impl AppScreen for Screen {
    fn handle_event(&mut self, event: Event) -> anyhow::Result<()> {
        if let Some(()) = self.back_button.handle_event(&event) {
            let portfolio_screen = Box::new(super::porfolio::Screen::new(self.command_tx.clone(), self.account.clone()));
            self.command_tx
                .send(AppCommand::SwitchScreen(portfolio_screen))
                .unwrap();
            return Ok(());
        }

        if let Some(reveal) = self.reveal_button.handle_event(&event) {
            self.mnemonic_words.masked = !reveal;
            return Ok(());
        }

        if let Some(()) = self.delete_button.handle_event(&event) {
            let delete_mnemonic_screen = Box::new(super::mnemonic_delete::Screen::new(
                self.command_tx.clone(), self.account.clone(), self.seed_phrase.clone()));
            self.command_tx
                .send(AppCommand::SwitchScreen(delete_mnemonic_screen))
                .unwrap();
            return Ok(());
        }
        Ok(())
    }

    fn render(&mut self, frame: &mut Frame) {
        let area = frame.area();
        let horizontal_padding = (area.width.saturating_sub(ACCESS_MNEMONIC_WIDTH)) / 2;

        let centered_area = Rect {
            x: horizontal_padding,
            y: area.y,
            width: ACCESS_MNEMONIC_WIDTH,
            height: area.height,
        };

        let content_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(0), // Fill height
                Constraint::Length(INTRO_HEIGHT),
                Constraint::Length(mnemonic::MNEMONIC_HEIGHT),
                Constraint::Length(BUTTONS_ROW_HEIGHT),
                Constraint::Min(0), // Fill height
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
