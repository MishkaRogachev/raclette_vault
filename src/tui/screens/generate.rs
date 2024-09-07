use std::sync::{atomic::AtomicBool, mpsc, Arc};
use ratatui::{
    crossterm::event::Event,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    widgets::Paragraph, Frame
};

use crate::{core::seed_phrase::SeedPhrase, tui::widgets::{common::Widget, mnemonic::MNEMONIC_HEIGHT}};
use crate::tui::app::AppCommand;
use super::super::widgets::{common, mnemonic};

const ONBOARDING_WIDTH: u16 = 80;
const INTRO_HEIGHT: u16 = 1;
const SWITCH_HEIGHT: u16 = 3;
const BUTTONS_ROW_HEIGHT: u16 = 3;

pub struct GeneratePhraseScreen {
    // FIXME: Keypair -> SeedPhrase
    seed_phrase: SeedPhrase,
    word_cnt_rx: mpsc::Receiver<bip39::MnemonicType>,
    reveal_flag: Arc<AtomicBool>,

    word_cnt_switch: common::Switch,
    reveal_words: mnemonic::RevealWords,
    back_button: common::Button,
    reveal_button: common::Button,
    hide_button: common::Button,
    next_button: common::Button,
}

impl GeneratePhraseScreen {
    pub fn new(command_tx: mpsc::Sender<AppCommand>) -> Self {
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
                    let welcome_screeen = Box::new(super::welcome::WelcomeScreen::new(command_tx.clone()));
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

        let next_button = {
            common::Button::new("Save keypair", Some('n'))
                .on_down(move || {
                    // command_tx.blocking_send(AppCommand::SwitchScreen(AppScreenType::Secure)).unwrap();
                })
        };

        Self {
            seed_phrase,
            word_cnt_rx,
            reveal_flag,
            word_cnt_switch,
            reveal_words,
            back_button,
            reveal_button,
            hide_button,
            next_button
        }
    }

    fn generate(&mut self, mtype: bip39::MnemonicType) {
        self.seed_phrase = SeedPhrase::generate(mtype);
        self.reveal_words = mnemonic::RevealWords::new(self.seed_phrase.to_words(), self.reveal_words.reveal_flag.clone());
    }
}

impl common::Widget for GeneratePhraseScreen {
    fn handle_event(&mut self, event: Event) -> Option<Event> {
        let revealed = self.reveal_flag.load(std::sync::atomic::Ordering::Relaxed);
        let mut controls: Vec<&mut dyn Widget> = vec![
            &mut self.word_cnt_switch,
            &mut self.back_button,
            if revealed { &mut self.hide_button } else { &mut self.reveal_button },
            &mut self.next_button
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

        let revealed = self.reveal_flag.load(std::sync::atomic::Ordering::Relaxed);
        let btn = if revealed { &mut self.hide_button } else { &mut self.reveal_button };
        btn.draw(frame, buttons_row[1]);

        self.next_button.draw(frame, buttons_row[2]);
    }
}
