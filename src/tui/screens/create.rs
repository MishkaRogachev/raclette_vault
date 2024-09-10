use std::sync::{atomic::AtomicBool, mpsc, Arc, Mutex};
use ratatui::{
    crossterm::event::Event,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    widgets::Paragraph, Frame
};

use crate::core::{key_pair::KeyPair, seed_phrase::SeedPhrase};
use crate::tui::app::AppCommand;
use crate::tui::widgets::{buttons, mnemonic, common};

const GENERATE_WIDTH: u16 = 80;
const INTRO_HEIGHT: u16 = 2;
const SWITCH_HEIGHT: u16 = 3;
const BUTTONS_ROW_HEIGHT: u16 = 3;

const INTRO_TEXT: &str = "Your master account will be based on the mnemonic seed phrase. \nYou may access it later in the app. Handle it with care!";

pub struct Screen {
    seed_phrase: Arc<Mutex<SeedPhrase>>,

    word_cnt_switch: buttons::SwitchButton,
    reveal_words: mnemonic::RevealWords,
    back_button: buttons::Button,
    reveal_button: buttons::SwapButton,
    secure_button: buttons::Button,
}

impl Screen {
    pub fn new(command_tx: mpsc::Sender<AppCommand>) -> Self {
        let seed_phrase = Arc::new(Mutex::new(SeedPhrase::generate(bip39::MnemonicType::Words12)));
        let reveal_flag = Arc::new(AtomicBool::new(false));
        let reveal_words = mnemonic::RevealWords::new(reveal_flag.clone());

        let word_cnt_switch = {
            let seed_phrase = seed_phrase.clone();
            buttons::SwitchButton::new("12 words", "24 words", Some('w'))
            .on_toggle(move |is_on| {
                seed_phrase.lock().unwrap().switch_mnemonic_type(
                if is_on { bip39::MnemonicType::Words24 } else { bip39::MnemonicType::Words12 });
            })
        };

        let back_button = {
            let command_tx = command_tx.clone();
            buttons::Button::new("Back", Some('b'))
                .on_down(move || {
                    let welcome_screeen = Box::new(super::welcome::Screen::new(command_tx.clone()));
                    command_tx.send(AppCommand::SwitchScreen(welcome_screeen)).unwrap();
                })
        };
        let reveal_button = {
            let reveal_flag = reveal_flag.clone();
            buttons::SwapButton::new(reveal_flag, "Reveal", Some('r'), "Hide", Some('h'))
        };
        let secure_button = {
            let seed_phrase = seed_phrase.clone();
            buttons::Button::new("Secure", Some('s'))
                .on_down(move || {
                    let keypair = KeyPair::from_seed(seed_phrase.lock().unwrap().to_seed("")).unwrap();
                    let secure_screeen = Box::new(super::secure::Screen::new(command_tx.clone(), keypair));
                    command_tx.send(AppCommand::SwitchScreen(secure_screeen)).unwrap();
                })
        };

        Self {
            seed_phrase,
            word_cnt_switch,
            reveal_words,
            back_button,
            reveal_button,
            secure_button
        }
    }
}

impl common::Widget for Screen {
    fn handle_event(&mut self, event: Event) -> Option<Event> {
        let mut controls: Vec<&mut dyn common::Widget> = vec![
            &mut self.word_cnt_switch,
            &mut self.back_button,
            &mut self.reveal_button,
            &mut self.secure_button
        ];
        controls.iter_mut().fold(Some(event), |event, button| {
            event.and_then(|e| button.handle_event(e))
        })
    }

    fn process(&mut self, frame: &mut Frame, area: Rect) {
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
                Constraint::Length(mnemonic::MNEMONIC_HEIGHT),
                Constraint::Length(BUTTONS_ROW_HEIGHT),
                Constraint::Min(0), // Fill height
            ])
            .split(centered_area);

        let intro_text = Paragraph::new(INTRO_TEXT)
            .style(Style::default().fg(Color::Yellow).bold())
            .alignment(Alignment::Center);
        frame.render_widget(intro_text, content_layout[1]);

        self.word_cnt_switch.process(frame, content_layout[2]);
        self.reveal_words.process(frame, content_layout[3]);

        let buttons_row = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(30),
            Constraint::Percentage(30),
            Constraint::Percentage(40),
        ])
        .split(content_layout[4]);

        self.back_button.process(frame, buttons_row[0]);
        self.reveal_button.process(frame, buttons_row[1]);
        self.secure_button.process(frame, buttons_row[2]);
    }
}
