use std::sync::{atomic::AtomicBool, mpsc, Arc};
use ratatui::{
    crossterm::event::Event,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    widgets::Paragraph, Frame
};

use crate::{core::seed_phrase::SeedPhrase, tui::widgets::{common::ControlTrait, mnemonic::MNEMONIC_HEIGHT}};
use crate::tui::app::{AppCommand, AppScreenType};
use super::super::widgets::{common, mnemonic};

const ONBOARDING_WIDTH: u16 = 80;
const INTRO_HEIGHT: u16 = 1;
const SWITCH_HEIGHT: u16 = 3;
const BUTTONS_ROW_HEIGHT: u16 = 3;

pub struct GeneratePhraseScreen {
    seed_phrase: SeedPhrase,
    word_cnt_rx: mpsc::Receiver<bip39::MnemonicType>,

    word_cnt_switch: common::Switch,
    reveal_words: mnemonic::RevealWords,
    back_button: common::Button,
    reveal_button: common::Button,
    next_button: common::Button,
}

impl GeneratePhraseScreen {
    pub fn new(command_tx: mpsc::Sender<AppCommand>) -> anyhow::Result<Self> {
        let seed_phrase = SeedPhrase::generate(bip39::MnemonicType::Words12);
        let reveal_flag = Arc::new(AtomicBool::new(false));
        let reveal_words = mnemonic::RevealWords::new(seed_phrase.to_words(), reveal_flag.clone());
        let (word_cnt_tx, word_cnt_rx) = mpsc::channel::<bip39::MnemonicType>();

        let word_cnt_switch = common::Switch::new("12 words", "24 words", Some('w'))
            .on_toggle(move |is_on| {
                word_cnt_tx.send(if is_on {
                    bip39::MnemonicType::Words24
                } else {
                    bip39::MnemonicType::Words12
                }).unwrap();
            });

        let back_button = {
            let command_tx = command_tx.clone();
            common::Button::new("Back", Some('b'))
                .on_down(move || {
                    command_tx.send(AppCommand::SwitchScreen(AppScreenType::Welcome)).unwrap();
                })
        };
        let reveal_button = {
            common::Button::new("Reveal", Some('r'))
                .on_up({
                    let reveal_flag = reveal_flag.clone();
                    move || { reveal_flag.store(false, std::sync::atomic::Ordering::Relaxed); }
                })
                .on_down(move || {
                    reveal_flag.store(true, std::sync::atomic::Ordering::Relaxed);
                })
                .warning()
        };

        let next_button = {
            common::Button::new("Save keypair", Some('n'))
                .on_down(move || {
                    // command_tx.blocking_send(AppCommand::SwitchScreen(AppScreenType::Secure)).unwrap();
                })
        };

        Ok(Self { seed_phrase, word_cnt_rx, word_cnt_switch, reveal_words, back_button, reveal_button, next_button })
    }

    fn generate(&mut self, mtype: bip39::MnemonicType) {
        self.seed_phrase = SeedPhrase::generate(mtype);
        self.reveal_words = mnemonic::RevealWords::new(self.seed_phrase.to_words(), self.reveal_words.reveal_flag.clone());
    }
}

impl common::ControlTrait for GeneratePhraseScreen {
    fn handle_event(&mut self, event: Event) -> Option<Event> {
        let mut controls: Vec<&mut dyn ControlTrait> = vec![
            &mut self.back_button,
            &mut self.reveal_button,
            &mut self.next_button,
            &mut self.word_cnt_switch,
        ];
        controls.iter_mut().fold(Some(event), |event, button| {
            event.and_then(|e| button.handle_event(e))
        })
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) {
        if let Ok(mtype) = self.word_cnt_rx.try_recv() {
            self.generate(mtype);
        }

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
                Constraint::Length(SWITCH_HEIGHT),
                Constraint::Length(MNEMONIC_HEIGHT),
                Constraint::Length(BUTTONS_ROW_HEIGHT),
                Constraint::Min(0), // Fill height
            ])
            .split(centered_area);

        let intro_text = Paragraph::new(
            "The seed phrase is a backup of your keypair. Handle with care.")
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
        self.reveal_button.draw(frame, buttons_row[1]);
        self.next_button.draw(frame, buttons_row[2]);
    }
}
