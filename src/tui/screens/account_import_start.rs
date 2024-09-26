use std::sync::mpsc;
use ratatui::{
    crossterm::event::Event,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    widgets::Paragraph,
    Frame,
};

use crate::core::seed_phrase::WordCount;
use crate::tui::app::{AppCommand, AppScreen};
use crate::tui::widgets::{buttons, ascii};

const IMPORT_WIDTH: u16 = 80;
const INTRO_HEIGHT: u16 = 3;
const SWITCH_HEIGHT: u16 = 3;
const OUTRO_HEIGHT: u16 = 2;
const BUTTONS_ROW_HEIGHT: u16 = 3;

const INTRO_TEXT: &str = "Strat importing your mnemonic seed phrase.\n Choose the correct number of words.";
const OUTRO_TEXT: &str = "Next, you will be prompted to enter your seed phrase words in the correct order.";

pub struct Screen {
    command_tx: mpsc::Sender<AppCommand>,
    word_count: WordCount,
    word_cnt_switch: buttons::MultiSwitch,
    back_button: buttons::Button,
    continue_button: buttons::Button,
}

impl Screen {
    pub fn new(command_tx: mpsc::Sender<AppCommand>) -> Self {
        let word_count = WordCount::Words12;
        let word_cnt_switch = buttons::MultiSwitch::new(vec![
                buttons::Button::new("12 words", Some('1')), buttons::Button::new("24 words", Some('2'))]);
        let back_button = buttons::Button::new("Back", Some('b'));
        let continue_button = buttons::Button::new("Continue", Some('c'));

        Self {
            command_tx,
            word_count,
            word_cnt_switch,
            back_button,
            continue_button,
        }
    }
}

#[async_trait::async_trait]
impl AppScreen for Screen {
    fn handle_event(&mut self, event: Event) -> anyhow::Result<()> {
        if let Some(is_on) = self.word_cnt_switch.handle_event(&event) {
            self.word_count = if is_on == 1 {
                WordCount::Words24
            } else {
                WordCount::Words12
            };
            return Ok(());
        }

        if let Some(()) = self.back_button.handle_event(&event) {
            let welcome_screen = Box::new(super::welcome::Screen::new(self.command_tx.clone()));
            self.command_tx
                .send(AppCommand::SwitchScreen(welcome_screen))
                .unwrap();
            return Ok(());
        }

        if let Some(()) = self.continue_button.handle_event(&event) {
            let import_words_screen = Box::new(super::account_import_words::Screen::new(
                self.command_tx.clone(), self.word_count, vec![], 0, false));
            self.command_tx
                .send(AppCommand::SwitchScreen(import_words_screen))
                .unwrap();
            return Ok(());
        }
        Ok(())
    }

    async fn update(&mut self) {}

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
                Constraint::Length(SWITCH_HEIGHT),
                Constraint::Fill(0), // Logo
                Constraint::Length(OUTRO_HEIGHT),
                Constraint::Length(BUTTONS_ROW_HEIGHT),
            ])
            .split(centered_area);

        let intro_text = Paragraph::new(INTRO_TEXT)
            .style(Style::default().fg(Color::Yellow).bold())
            .alignment(Alignment::Center);
        frame.render_widget(intro_text, content_layout[0]);

        self.word_cnt_switch.render(frame, content_layout[1]);

        let logo = Paragraph::new(ascii::KEYS)
            .style(Style::default().fg(Color::Yellow))
            .alignment(Alignment::Center);
        frame.render_widget(logo, content_layout[2]);

        let outro_text = Paragraph::new(OUTRO_TEXT)
            .style(Style::default().fg(Color::Yellow).bold())
            .alignment(Alignment::Center);
        frame.render_widget(outro_text, content_layout[3]);

        let buttons_row = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ])
            .split(content_layout[4]);

        self.back_button.render(frame, buttons_row[0]);
        self.continue_button.render(frame, buttons_row[1]);
    }
}
