use std::sync::mpsc;
use ratatui::{
    crossterm::event::Event,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    widgets::Paragraph,
    Frame
};
use zeroize::Zeroizing;

use crate::core::seed_phrase::WordCount;
use crate::tui::{app::{AppCommand, AppScreen}, widgets::{bars, buttons, focus::{self, Focusable}, inputs}};

const IMPORT_WIDTH: u16 = 80;
const INTRO_HEIGHT: u16 = 1;
const PROGRESS_HEIGHT: u16 = 3;
const INPUT_LABEL_HEIGHT: u16 = 1;
const INPUT_HEIGHT: u16 = 3;
const BUTTONS_ROW_HEIGHT: u16 = 3;

const INTRO_TEXT: &str = "Importing your seed phrase";
const LABEL_TEXT: &str = "Enter word";

pub struct Screen {
    command_tx: mpsc::Sender<AppCommand>,
    word_count: WordCount,
    words: Vec<Zeroizing<String>>,
    index: usize,

    bar: bars::HProgress,
    input: inputs::Input,
    back_button: buttons::Button,
    reveal_button: buttons::SwapButton,
    next_button: buttons::Button,
}

impl Screen {
    pub fn new(command_tx: mpsc::Sender<AppCommand>, word_count: WordCount, words: Vec<Zeroizing<String>>, index: usize, revealed: bool) -> Self {
        let bar = bars::HProgress::new(0, word_count as u64, index as u64);
        let mut input = inputs::Input::new("Enter word").masked();
        let back_button = buttons::Button::new("Back", Some('b'));
        let mut reveal_button = buttons::SwapButton::new(
            buttons::Button::new("Reveal", Some('r')).warning(),
            buttons::Button::new("Hide", Some('h')).primary(),
        );
        let mut next_button = buttons::Button::new("Next", Some('n'));

        input.set_focused(true);

        if revealed {
            input.masked = false;
            reveal_button.swap();
        }

        if index < words.len() {
            input.value = words[index].clone();
        } else {
            next_button.disabled = true;
        }

        Self {
            command_tx,
            word_count,
            words,
            index,
            bar,
            input,
            back_button,
            reveal_button,
            next_button,
        }
    }
}

#[async_trait::async_trait]
impl AppScreen for Screen {
    fn handle_event(&mut self, event: Event) -> anyhow::Result<bool> {
        let revealed = !self.input.masked;
        let next_action = |word: &str| {
            let mut words = self.words.clone();

            if self.index < words.len() {
                words[self.index] = Zeroizing::new(word.to_string());
            } else {
                words.push(Zeroizing::new(word.to_string()));
            }

            if self.index + 1 == self.word_count as usize {
                let finalize_screen = Box::new(super::account_import_finalize::Screen::new(
                    self.command_tx.clone(), words, self.word_count));
                self.command_tx.send(AppCommand::SwitchScreen(finalize_screen)).unwrap();
            } else {
                let import_screen = Box::new(super::account_import_words::Screen::new(
                    self.command_tx.clone(), self.word_count, words, self.index + 1, revealed));
                self.command_tx.send(AppCommand::SwitchScreen(import_screen)).unwrap();
            }
        };

        if let Some(event) = focus::handle_scoped_event(&mut [&mut self.input], &event) {
            if let focus::FocusableEvent::FocusFinished = event {
                if !self.input.value.is_empty() {
                    next_action(&self.input.value);
                    return Ok(true);
                }
            }
            self.next_button.disabled = self.input.value.is_empty();
            return Ok(false);
        }

        if let Some(()) = self.back_button.handle_event(&event) {
            if self.index > 0 {
                let import_screen = Box::new(super::account_import_words::Screen::new(
                    self.command_tx.clone(), self.word_count, self.words.clone(), self.index - 1, revealed));
                self.command_tx.send(AppCommand::SwitchScreen(import_screen)).unwrap();
                return Ok(true);
            }
            let welcome_screen = Box::new(super::welcome::Screen::new(self.command_tx.clone()));
            self.command_tx.send(AppCommand::SwitchScreen(welcome_screen)).unwrap();
        }

        if let Some(reveal) = self.reveal_button.handle_event(&event) {
            self.input.masked = !reveal;
            return Ok(true);
        }

        if let Some(()) = self.next_button.handle_event(&event) {
            next_action(&self.input.value);
            return Ok(true);
        }
        Ok(false)
    }

    async fn update(&mut self) {}

    fn render(&mut self, frame: &mut Frame, area: Rect) {
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
                Constraint::Min(0), // Fill height
                Constraint::Length(PROGRESS_HEIGHT),
                Constraint::Min(0), // Fill height
                Constraint::Length(INPUT_LABEL_HEIGHT),
                Constraint::Length(INPUT_HEIGHT),
                Constraint::Min(0), // Fill height
                Constraint::Length(BUTTONS_ROW_HEIGHT),
            ])
            .split(centered_area);

        let intro_text = Paragraph::new(INTRO_TEXT)
            .style(Style::default().fg(Color::Yellow).bold())
            .alignment(Alignment::Center);
        frame.render_widget(intro_text, content_layout[0]);

        self.bar.render(frame, content_layout[2]);

        let label = Paragraph::new(format!("{} {}", LABEL_TEXT, self.index + 1))
            .style(Style::default().fg(Color::Yellow).bold())
            .alignment(Alignment::Center);
        frame.render_widget(label, content_layout[4]);

        self.input.render(frame, content_layout[5]);

        let buttons_row = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(30),
                Constraint::Percentage(30),
                Constraint::Percentage(40),
            ])
            .split(content_layout[7]);

        self.back_button.render(frame, buttons_row[0]);
        self.reveal_button.render(frame, buttons_row[1]);
        self.next_button.render(frame, buttons_row[2]);
    }
}
