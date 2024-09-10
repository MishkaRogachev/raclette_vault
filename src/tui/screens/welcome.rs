
use std::sync::mpsc;
use ratatui::{
    crossterm::event::Event,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    widgets::Paragraph, Frame
};

use crate::tui::app::AppCommand;
use crate::tui::widgets::{buttons, ascii, common};

const WELCOME_WIDTH: u16 = 60;
const LOGO_HEIGHT: u16 = 20;
const WARNING_HEIGHT: u16 = 1;
const BUTTONS_ROW_HEIGHT: u16 = 3;

pub struct Screen {
    quit_button: buttons::Button,
    create_button: buttons::Button,
}

impl Screen {
    pub fn new(command_tx: mpsc::Sender<AppCommand>) -> Self {
        let quit_button = {
            let command_tx = command_tx.clone();
            buttons::Button::new("Quit", Some('q'))
                .on_down(move || { command_tx.send(AppCommand::Quit).unwrap(); })
        };
        let create_button = {
            buttons::Button::new("Create Master Account", Some('c'))
                .on_down(move || {
                    let create_screeen = Box::new(super::create::Screen::new(command_tx.clone()));
                    command_tx.send(AppCommand::SwitchScreen(create_screeen)).unwrap();
                })
        };

        Self { quit_button, create_button }
    }
}

impl common::Widget for Screen {
    fn handle_event(&mut self, event: Event) -> Option<Event> {
        [&mut self.quit_button, &mut self.create_button]
            .iter_mut().fold(Some(event), |event, button| {
            event.and_then(|e| button.handle_event(e))
        })
    }

    fn process(&mut self, frame: &mut Frame, area: Rect) {
        let horizontal_padding = (area.width.saturating_sub(WELCOME_WIDTH)) / 2;

        let centered_area = Rect {
            x: horizontal_padding,
            y: area.y,
            width: WELCOME_WIDTH,
            height: area.height,
        };

        let content_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(0), // Fill height
                Constraint::Length(LOGO_HEIGHT),
                Constraint::Length(WARNING_HEIGHT),
                Constraint::Length(BUTTONS_ROW_HEIGHT),
                Constraint::Min(0), // Fill height
            ])
            .split(centered_area);

        let logo = Paragraph::new(ascii::BIG_LOGO)
            .style(Style::default().fg(Color::Yellow))
            .alignment(Alignment::Center);
        frame.render_widget(logo, content_layout[1]);

        let warning_text = Paragraph::new("Please don't use this wallet for real crypto!")
            .style(Style::default().fg(Color::Red).bold())
            .alignment(Alignment::Center);
        frame.render_widget(warning_text, content_layout[2]);

        let buttons_row = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(30),
                Constraint::Percentage(70),
            ])
            .split(content_layout[3]);

        self.quit_button.process(frame, buttons_row[0]);
        self.create_button.process(frame, buttons_row[1]);
    }
}
