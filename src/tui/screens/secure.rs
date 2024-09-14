
use std::sync::{atomic::AtomicBool, mpsc, Arc, Mutex};
use zeroize::Zeroizing;
use ratatui::{
    crossterm::event::Event,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    widgets::Paragraph, Frame
};

use crate::{core::seed_phrase, service::account};
use crate::tui::app::AppCommand;

use crate::tui::widgets::{common, focus, buttons, inputs};

const SECURE_WIDTH: u16 = 80;
const INTRO_HEIGHT: u16 = 2;
const INPUT_LABEL_HEIGHT: u16 = 1;
const INPUT_HEIGHT: u16 = 3;
const TIP_HEIGHT: u16 = 1;
const BUTTONS_ROW_HEIGHT: u16 = 3;

const INTRO_TEXT: &str = "Your master account keypair was created. Now let's secure it!";
const FIRST_LABEL_TEXT: &str = "Enter password. It will not be stored anywhere.";
const SECOND_LABEL_TEXT: &str = "Please, confirm your password.";
const TIP_TEXT: &str = "Tip: Use [tab] to switch focus and [esc] to reset selection.";

pub struct Screen {
    first_input: inputs::Input,
    second_input: inputs::Input,
    back_button: buttons::Button,
    reveal_button: buttons::SwapButton,
    save_button: buttons::Button,
}

impl Screen {
    pub fn new(command_tx: mpsc::Sender<AppCommand>, seed_phrase: seed_phrase::SeedPhrase) -> Self {
        let reveal_flag = Arc::new(AtomicBool::new(false));
        let password = Arc::new(Mutex::new(Zeroizing::new(String::new())));

        let first_input = {
            let password = password.clone();
            inputs::Input::new("Enter password")
                .mask(reveal_flag.clone())
                .on_enter(move |value| {
                    *password.lock().unwrap() = Zeroizing::new(value.to_string());
                })
        };
        let second_input = inputs::Input::new("Confirm password")
            .mask(reveal_flag.clone());

        let back_button = {
            let command_tx = command_tx.clone();
            buttons::Button::new("Back", Some('b'))
                .on_down(move || {
                    let create_screeen = Box::new(super::create::Screen::new(command_tx.clone()));
                    command_tx.send(AppCommand::SwitchScreen(create_screeen)).unwrap();
                })
        };
        let reveal_button = buttons::SwapButton::new(
            reveal_flag, "Reveal", Some('r'), "Hide", Some('h'));
        let save_button = {
            buttons::Button::new("Save & Finish", Some('s'))
                .on_down(move || {
                    let password = password.lock().unwrap().clone();
                    let account = account::Account::create(&seed_phrase, &password).expect("Fatal issue with creating an account");
                    let home_screeen = Box::new(super::home::Screen::new(command_tx.clone(), account));
                    command_tx.send(AppCommand::SwitchScreen(home_screeen)).unwrap();
                })
        };

        Self { first_input, second_input, back_button, reveal_button, save_button }
    }
}

impl common::Widget for Screen {
    fn handle_event(&mut self, event: Event) -> Option<Event> {
        let event = focus::handle_event(&mut [&mut self.first_input, &mut self.second_input], event);
        match event {
            Some(event) => {
                let mut controls: Vec<&mut dyn common::Widget> = vec![
                    &mut self.back_button,
                    &mut self.reveal_button,
                    &mut self.save_button
                ];
                controls.iter_mut().fold(Some(event), |event, button| {
                    event.and_then(|e| button.handle_event(e))
                })
            },
            None => None
        }
    }

    fn process(&mut self, frame: &mut Frame, area: Rect) {
        let first_password = &self.first_input.value;
        let second_password = &self.second_input.value;

        if first_password.is_empty() {
            self.first_input.color = Color::Red;
            self.second_input.color = Color::Red;
            self.save_button.disabled = false;
        } else {
            self.first_input.color = Color::Yellow;

            if *first_password != *second_password {
                self.second_input.color = Color::Red;
                self.save_button.disabled = true;
            } else {
                self.second_input.color = Color::Yellow;
                self.save_button.disabled = false;
            }
        }

        let horizontal_padding = (area.width.saturating_sub(SECURE_WIDTH)) / 2;

        let centered_area = Rect {
            x: horizontal_padding,
            y: area.y,
            width: SECURE_WIDTH,
            height: area.height,
        };

        let content_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(0), // Fill height
                Constraint::Length(INTRO_HEIGHT),
                Constraint::Length(INPUT_LABEL_HEIGHT),
                Constraint::Length(INPUT_HEIGHT),
                Constraint::Length(INPUT_LABEL_HEIGHT),
                Constraint::Length(INPUT_HEIGHT),
                Constraint::Length(TIP_HEIGHT),
                Constraint::Length(BUTTONS_ROW_HEIGHT),
                Constraint::Min(0), // Fill height
            ])
            .split(centered_area);

        let intro_text = Paragraph::new(INTRO_TEXT)
            .style(Style::default().fg(Color::Yellow).bold())
            .alignment(Alignment::Center);
        frame.render_widget(intro_text, content_layout[1]);

        let first_label = Paragraph::new(FIRST_LABEL_TEXT)
            .style(Style::default().fg(Color::Yellow).bold())
            .alignment(Alignment::Center);
        frame.render_widget(first_label, content_layout[2]);

        self.first_input.process(frame, content_layout[3]);

        let second_label = Paragraph::new(SECOND_LABEL_TEXT)
            .style(Style::default().fg(Color::Yellow).bold())
            .alignment(Alignment::Center);
        frame.render_widget(second_label, content_layout[4]);

        self.second_input.process(frame, content_layout[5]);

        let tip_text = Paragraph::new(TIP_TEXT)
            .style(Style::default().fg(Color::Yellow).bold())
            .alignment(Alignment::Center);
        frame.render_widget(tip_text, content_layout[6]);

        let buttons_row = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(30),
            Constraint::Percentage(30),
            Constraint::Percentage(40),
        ])
        .split(content_layout[7]);

        self.back_button.process(frame, buttons_row[0]);
        self.reveal_button.process(frame, buttons_row[1]);
        self.save_button.process(frame, buttons_row[2]);
    }
}
