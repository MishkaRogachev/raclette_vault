
use std::sync::mpsc;
use ratatui::{
    crossterm::event::Event,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    widgets::Paragraph, Frame
};

use crate::tui::app::{AppCommand, AppScreenType};
use super::super::{widgets::common, logo};

const WELLCOME_HEIGHT: u16 = 1;
const WARNING_HEIGHT: u16 = 1;
const BUTTONS_ROW_HEIGHT: u16 = 3;

pub struct WelcomeScreen {
    quit_button: common::Button,
    generate_button: common::Button,
}

impl WelcomeScreen {
    pub fn new(command_tx: mpsc::Sender<AppCommand>) -> anyhow::Result<Self> {
        let quit_button = {
            let command_tx = command_tx.clone();
            common::Button::new("Quit", Some('q'))
                .action(move || { command_tx.send(AppCommand::Quit).unwrap(); })
        };
        let generate_button = {
            common::Button::new("Create Account", Some('o'))
                .action(move || {
                    command_tx.send(AppCommand::SwitchScreen(AppScreenType::Generate)).unwrap();
                })
        };

        Ok(Self { quit_button, generate_button })
    }
}

impl common::Widget for WelcomeScreen {
    fn handle_event(&mut self, event: Event) -> Option<Event> {
        let event = self.quit_button.handle_event(event);
        if let Some(event) = event {
            return self.generate_button.handle_event(event);
        }
        None
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) {
        let horizontal_padding = (area.width.saturating_sub(logo::BIG_LOGO_WIDTH)) / 2;

        let centered_area = Rect {
            x: horizontal_padding,
            y: area.y,
            width: logo::BIG_LOGO_WIDTH,
            height: area.height,
        };

        let content_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(0), // Fill height
                Constraint::Length(logo::BIG_LOGO_HEIGHT),
                Constraint::Length(WELLCOME_HEIGHT),
                Constraint::Length(WARNING_HEIGHT),
                Constraint::Length(BUTTONS_ROW_HEIGHT),
                Constraint::Min(0), // Fill height
            ])
            .split(centered_area);

        logo::big_logo(content_layout[1], frame);

        let welcome_text = Paragraph::new("Welcome to Raclette Vault!")
            .style(Style::default().fg(Color::Yellow))
            .alignment(Alignment::Center);
        frame.render_widget(welcome_text, content_layout[2]);

        let warning_text = Paragraph::new("Please don't use this wallet for real crypto!")
            .style(Style::default().fg(Color::Red).bold())
            .alignment(Alignment::Center);
        frame.render_widget(warning_text, content_layout[3]);

        let buttons_row = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(30),
                Constraint::Percentage(70),
            ])
            .split(content_layout[4]);

        self.quit_button.draw(frame, buttons_row[0]);
        self.generate_button.draw(frame, buttons_row[1]);
    }
}
