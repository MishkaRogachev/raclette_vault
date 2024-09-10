
use std::sync::{mpsc, Arc, Mutex};
use ratatui::{
    crossterm::event::Event,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    widgets::Paragraph, Frame
};

use crate::core::key_pair::KeyPair;
use crate::tui::app::AppCommand;

use crate::tui::widgets::{common, buttons, inputs};

const SECURE_WIDTH: u16 = 80;
const INTRO_HEIGHT: u16 = 3;
const INPUT_HEIGHT: u16 = 3;
const MID_HEIGHT: u16 = 3;
const OUTRO_HEIGHT: u16 = 3;
const BUTTONS_ROW_HEIGHT: u16 = 3;

const INTRO_TEXT: &str = "Your master account keypair was created. Now let's secure it.\nPlease enter a password to encrypt your keypair.";

pub struct Screen {
    keypair: KeyPair,
    first_input: inputs::Input,
    back_button: buttons::Button,
    save_button: buttons::Button,
}

impl Screen {
    pub fn new(command_tx: mpsc::Sender<AppCommand>, keypair: KeyPair) -> Self {
        let first_password = Arc::new(Mutex::new(String::new()));
        //let second_password = Arc::new(Mutex::new(String::new()));

        let mut first_input = inputs::Input::new("Enter password")
            .on_enter(move |value| { *first_password.lock().unwrap() = value;})
            .masked("*");

        first_input.focused = true;

        let back_button = {
            let command_tx = command_tx.clone();
            buttons::Button::new("Back", Some('b'))
                .on_down(move || {
                    let create_screeen = Box::new(super::create::Screen::new(command_tx.clone()));
                    command_tx.send(AppCommand::SwitchScreen(create_screeen)).unwrap();
                })
        };

        let save_button = {
            //let command_tx = command_tx.clone();
            buttons::Button::new("Save & Finish", Some('s'))
                .on_down(move || {
                    // let home_screeen = Box::new(super::welcome::HomeScreen::new(command_tx.clone(), keypair.clone()));
                    // command_tx.send(AppCommand::SwitchScreen(home_screeen)).unwrap();
                })
        };

        Self { keypair, first_input, back_button, save_button }
    }
}

impl common::Widget for Screen {
    fn handle_event(&mut self, event: Event) -> Option<Event> {
        if self.first_input.focused {
            return self.first_input.handle_event(event);
        }

        vec![&mut self.back_button, &mut self.save_button].iter_mut().fold(Some(event), |event, button| {
            event.and_then(|e| button.handle_event(e))
        })
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) {
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
                Constraint::Length(INPUT_HEIGHT),
                Constraint::Length(BUTTONS_ROW_HEIGHT),
                Constraint::Min(0), // Fill height
            ])
            .split(centered_area);

        let intro_text = Paragraph::new(INTRO_TEXT)
            .style(Style::default().fg(Color::Yellow).bold())
            .alignment(Alignment::Center);
        frame.render_widget(intro_text, content_layout[1]);

        self.first_input.draw(frame, content_layout[2]);

        let buttons_row = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(content_layout[3]);

        self.back_button.draw(frame, buttons_row[0]);
        self.save_button.draw(frame, buttons_row[1]);
    }
}
