use std::sync::mpsc;
use ratatui::{
    crossterm::event::Event,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    widgets::Paragraph, Frame
};

use crate::core::seed_phrase::SeedPhrase;
use crate::tui::app::{AppCommand, AppScreenType};
use super::super::widgets::{common, mnemonic};

const ONBOARDING_WIDTH: u16 = 60;
const INTRO_HEIGHT: u16 = 2;
const WARNING_HEIGHT: u16 = 1;
const OUTRO_HEIGHT: u16 = 2;
const BUTTONS_ROW_HEIGHT: u16 = 3;

pub struct GenerateKeypairScreen {
    seed_phrase: SeedPhrase,
    reveal_words: mnemonic::RevealWords,
    back_button: common::Button,
    next_button: common::Button,
}

impl GenerateKeypairScreen {
    pub fn new(command_tx: mpsc::Sender<AppCommand>) -> anyhow::Result<Self> {
        let seed_phrase = SeedPhrase::generate_12_words();
        let reveal_words = mnemonic::RevealWords::new(seed_phrase.to_words());

        let back_button = {
            let command_tx = command_tx.clone();
            common::Button::new("Back", Some('b'))
                .action(move || {
                    command_tx.send(AppCommand::SwitchScreen(AppScreenType::Welcome)).unwrap();
                })
        };
        let next_button = {
            common::Button::new("Save keypair", Some('n'))
                .action(move || {
                    // command_tx.blocking_send(AppCommand::SwitchScreen(AppScreenType::Secure)).unwrap();
                })
        };

        Ok(Self { seed_phrase, reveal_words, back_button, next_button })
    }
}

impl common::Widget for GenerateKeypairScreen {
    fn handle_event(&mut self, event: Event) -> Option<Event> {
        let event = self.back_button.handle_event(event);
        if let Some(event) = event {
            return self.next_button.handle_event(event);
        }
        None
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) {
        let horizontal_padding = (area.width.saturating_sub(ONBOARDING_WIDTH)) / 2;

        let centered_area = Rect {
            x: horizontal_padding,
            y: area.y,
            width: ONBOARDING_WIDTH,
            height: area.height,
        };

        let content_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(0), // Fill height
                Constraint::Length(INTRO_HEIGHT),
                Constraint::Length(self.reveal_words.height()),
                Constraint::Length(WARNING_HEIGHT),
                Constraint::Length(OUTRO_HEIGHT),
                Constraint::Length(BUTTONS_ROW_HEIGHT),
                Constraint::Min(0), // Fill height
            ])
            .split(centered_area);

        let intro_text = Paragraph::new(
            "Your seed phrase has been successfully created!")
            .style(Style::default().fg(Color::Yellow).bold())
            .alignment(Alignment::Center);
        frame.render_widget(intro_text, content_layout[1]);

        self.reveal_words.draw(frame, content_layout[2]);

        let warning_text = Paragraph::new(
            "Be cautious when revealing seen phrase!")
            .style(Style::default().fg(Color::Red).bold())
            .alignment(Alignment::Center);
        frame.render_widget(warning_text, content_layout[3]);

        let outro_text = Paragraph::new(
            "You’ll be able to access it later in the app.")
            .style(Style::default().fg(Color::Yellow).bold())
            .alignment(Alignment::Center);
        frame.render_widget(outro_text, content_layout[4]);

        let buttons_row = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(30),
            Constraint::Percentage(70),
        ])
        .split(content_layout[5]);

        self.back_button.draw(frame, buttons_row[0]);
        self.next_button.draw(frame, buttons_row[1]);
    }
}
