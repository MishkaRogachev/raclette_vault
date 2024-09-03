use std::sync::{atomic::AtomicBool, Arc};
use ratatui::{
    crossterm::event::Event,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    widgets::Paragraph, Frame
};

use super::super::{common, logo};

const WELLCOME_HEIGHT: u16 = 1;
const WARNING_HEIGHT: u16 = 1;
const BUTTONS_ROW_HEIGHT: u16 = 3;

pub struct WelcomeScreen {
    quit_button: common::Button,
    create_account_button: common::Button,
}

impl WelcomeScreen {
    pub fn new(shutdown_handle: Arc<AtomicBool>) -> Self {
        let quit_button = common::Button::new("Quit", Some('q'))
            .action(move || { shutdown_handle.store(true, std::sync::atomic::Ordering::Relaxed); });
        let create_account_button = common::Button::new("Create User Account", Some('c'));

        Self { quit_button, create_account_button }
    }
}

impl common::Widget for WelcomeScreen {
    fn handle_event(&mut self, event: Event) -> Option<Event> {
        let event = self.quit_button.handle_event(event);
        if let Some(event) = event {
            return self.create_account_button.handle_event(event);
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
    
        // Vertical layout for the content
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
    
        // Horizontal layout for the buttons within the logo width
        let buttons_row = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(30),
                Constraint::Percentage(70),
            ])
            .split(content_layout[4]);
    
        // Render the logo
        logo::big_logo(content_layout[1], frame);
    
        // Render the welcome text
        let welcome_text = Paragraph::new("Welcome to Raclette Vault!")
            .style(Style::default().fg(Color::Yellow))
            .alignment(Alignment::Center);
        frame.render_widget(welcome_text, content_layout[2]);
    
        // Render the warning text
        let warning_text = Paragraph::new("Please don't use this wallet for real crypto!")
            .style(Style::default().fg(Color::Red).bold())
            .alignment(Alignment::Center);
        frame.render_widget(warning_text, content_layout[3]);

        self.quit_button.draw(frame, buttons_row[0]);
        self.create_account_button.draw(frame, buttons_row[1]);
    }
}
