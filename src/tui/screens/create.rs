use std::sync::{atomic::AtomicBool, mpsc, Arc, Mutex};
use ratatui::{
    crossterm::event::Event,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    widgets::Paragraph, Frame
};

use crate::core::{key_pair::KeyPair, seed_phrase::SeedPhrase};
use crate::tui::widgets::{common::Widget, mnemonic::MNEMONIC_HEIGHT};
use crate::tui::app::AppCommand;
use super::super::widgets::{common, mnemonic};

const GENERATE_WIDTH: u16 = 80;
const INTRO_HEIGHT: u16 = 2;
const SWITCH_HEIGHT: u16 = 3;
const BUTTONS_ROW_HEIGHT: u16 = 3;

pub struct Screen {
    seed_phrase: Arc<Mutex<SeedPhrase>>,
    reveal_flag: Arc<AtomicBool>,

    word_cnt_switch: common::Switch,
    reveal_words: mnemonic::RevealWords,
    back_button: common::Button,
    reveal_button: common::Button,
    hide_button: common::Button,
    secure_button: common::Button,
}

impl Screen {
    pub fn new(command_tx: mpsc::Sender<AppCommand>) -> Self {
        let seed_phrase = Arc::new(Mutex::new(SeedPhrase::generate(bip39::MnemonicType::Words12)));
        let reveal_flag = Arc::new(AtomicBool::new(false));
        let reveal_words = mnemonic::RevealWords::new(reveal_flag.clone());

        let word_cnt_switch = {
            let seed_phrase = seed_phrase.clone();
            common::Switch::new("12 words", "24 words", Some('w'))
            .on_toggle(move |is_on| {
                seed_phrase.lock().unwrap().switch_mnemonic_type(
                if is_on {
                    bip39::MnemonicType::Words24
                } else {
                    bip39::MnemonicType::Words12
                });
            })
        };

        let back_button = {
            let command_tx = command_tx.clone();
            common::Button::new("Back", Some('b'))
                .on_down(move || {
                    let welcome_screeen = Box::new(super::welcome::Screen::new(command_tx.clone()));
                    command_tx.send(AppCommand::SwitchScreen(welcome_screeen)).unwrap();
                })
        };
        let reveal_button = {
            let reveal_flag = reveal_flag.clone();
            common::Button::new("Reveal", Some('r'))
                .on_down(move || {
                    reveal_flag.store(true, std::sync::atomic::Ordering::Relaxed);
                })
                .warning()
        };
        let hide_button = {
            let reveal_flag = reveal_flag.clone();
            common::Button::new("Hide", Some('h'))
                .on_down(move || {
                    reveal_flag.store(false, std::sync::atomic::Ordering::Relaxed);
                })
                .primary()
        };

        let secure_button = {
            let seed_phrase = seed_phrase.clone();
            common::Button::new("Secure", Some('n'))
                .on_down(move || {
                    let keypair = KeyPair::from_seed(seed_phrase.lock().unwrap().to_seed("")).unwrap();
                    let secure_screeen = Box::new(super::secure::Screen::new(command_tx.clone(), keypair));
                    command_tx.send(AppCommand::SwitchScreen(secure_screeen)).unwrap();
                })
        };

        Self {
            seed_phrase,
            reveal_flag,
            word_cnt_switch,
            reveal_words,
            back_button,
            reveal_button,
            hide_button,
            secure_button
        }
    }
}

impl common::Widget for Screen {
    fn handle_event(&mut self, event: Event) -> Option<Event> {
        let revealed = self.reveal_flag.load(std::sync::atomic::Ordering::Relaxed);
        let mut controls: Vec<&mut dyn Widget> = vec![
            &mut self.word_cnt_switch,
            &mut self.back_button,
            if revealed { &mut self.hide_button } else { &mut self.reveal_button },
            &mut self.secure_button
        ];
        controls.iter_mut().fold(Some(event), |event, button| {
            event.and_then(|e| button.handle_event(e))
        })
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) {
        self.reveal_words.set_words(self.seed_phrase.lock().unwrap().to_words());

        let horizontal_padding = (area.width.saturating_sub(GENERATE_WIDTH)) / 2;

        let centered_area = Rect {
            x: horizontal_padding,
            y: area.y,
            width: GENERATE_WIDTH,
            height: area.height,
        };

        let content_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(0), // Fill height
                Constraint::Length(INTRO_HEIGHT),
                Constraint::Length(SWITCH_HEIGHT),
                Constraint::Length(MNEMONIC_HEIGHT),
                Constraint::Length(BUTTONS_ROW_HEIGHT),
                Constraint::Min(0), // Fill height
            ])
            .split(centered_area);

        let intro_text = Paragraph::new(
            "Your master account will be based on the mnemonic seed phrase. \nYou may access it later in the app. Handle with care!")
            .style(Style::default().fg(Color::Yellow).bold())
            .alignment(Alignment::Center);
        frame.render_widget(intro_text, content_layout[1]);

        self.word_cnt_switch.draw(frame, content_layout[2]);
        self.reveal_words.draw(frame, content_layout[3]);

        let buttons_row = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(30),
            Constraint::Percentage(30),
            Constraint::Percentage(40),
        ])
        .split(content_layout[4]);

        self.back_button.draw(frame, buttons_row[0]);

        let revealed = self.reveal_flag.load(std::sync::atomic::Ordering::Relaxed);
        let btn = if revealed { &mut self.hide_button } else { &mut self.reveal_button };
        btn.draw(frame, buttons_row[1]);

        self.secure_button.draw(frame, buttons_row[2]);
    }
}
