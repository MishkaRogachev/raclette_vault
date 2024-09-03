use std::sync::{atomic::AtomicBool, Arc};

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    widgets::Paragraph, Frame
};
use super::super::{common, logo};

const WELLCOME_HEIGHT: u16 = 1;
const WARNING_HEIGHT: u16 = 1;
const BUTTONS_ROW_HEIGHT: u16 = 3;

#[derive(Debug, Clone)]
pub struct WelcomeScreen {
    shutdown_handle: Arc<AtomicBool>
}

impl WelcomeScreen {
    pub fn new(shutdown_handle: Arc<AtomicBool>) -> Self {
        Self { shutdown_handle }
    }
}

impl common::Screen for WelcomeScreen {
    fn handle_key_event(&self, key_event: ratatui::crossterm::event::KeyEvent) {
        match key_event.code {
            ratatui::crossterm::event::KeyCode::Char('q') => {
                self.shutdown_handle.store(true, std::sync::atomic::Ordering::Relaxed);
            },
            ratatui::crossterm::event::KeyCode::Char('c') => {
                // TODO: next screen
            },
            _ => {}
        }
    }

    fn handle_mouse_event(&self, _mouse_event: ratatui::crossterm::event::MouseEvent) {
        // TODO: mouse input
    }

    fn draw(&self, frame: &mut Frame, area: Rect) {
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
    
        // Render quit button
        let shutdown_handle = self.shutdown_handle.clone();
        let quit_button = common::Button::new("Quit", Some('q'))
            .action(move || { shutdown_handle.store(true, std::sync::atomic::Ordering::Relaxed); });
        frame.render_widget(quit_button, buttons_row[0]);
    
        // Render create account
        let create_account_button = common::Button::new("Create User Account", Some('c'));
        frame.render_widget(create_account_button, buttons_row[1]);
    }
}
