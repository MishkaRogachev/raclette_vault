use std::sync::mpsc;

use ratatui::crossterm::event::Event;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::Frame;

use crate::core::key_pair::KeyPair;
use crate::tui::{widgets::{buttons, common}, app::AppCommand};

const HOME_WIDTH: u16 = 60;
const BUTTONS_ROW_HEIGHT: u16 = 3;

pub struct Screen {
    kay_pair: KeyPair,

    quit_button: buttons::Button,
}

impl Screen {
    pub fn new(command_tx: mpsc::Sender<AppCommand>, kay_pair: KeyPair) -> Self {
        let quit_button = buttons::Button::new("Quit", Some('q'))
            .on_down(move || {
                command_tx.send(AppCommand::Quit).unwrap();
            });

        Self {
            kay_pair,
            quit_button,
        }
    }
}

impl common::Widget for Screen {
    fn handle_event(&mut self, event: Event) -> Option<Event> {
        [&mut self.quit_button, ]
            .iter_mut().fold(Some(event), |event, button| {
            event.and_then(|e| button.handle_event(e))
        })
    }

    fn process(&mut self, frame: &mut Frame, area: Rect) {
        let horizontal_padding = (area.width.saturating_sub(HOME_WIDTH)) / 2;

        let centered_area = Rect {
            x: horizontal_padding,
            y: area.y,
            width: HOME_WIDTH,
            height: area.height,
        };

        let content_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(0), // Fill height
                Constraint::Length(BUTTONS_ROW_HEIGHT),
                Constraint::Min(0), // Fill height
            ])
            .split(centered_area);

        let buttons_row = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(30),
                Constraint::Percentage(70),
            ])
            .split(content_layout[1]);

        self.quit_button.process(frame, buttons_row[0]);
    }
}