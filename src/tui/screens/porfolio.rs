use std::sync::mpsc;
use ratatui::{
    crossterm::event::Event,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    widgets::Paragraph, Frame
};

use crate::service::account::Account;
use crate::tui::{widgets::buttons, app::{AppCommand, AppScreen}};

const HOME_WIDTH: u16 = 60;
const INTRO_HEIGHT: u16 = 1;
const ACCOUNT_HEIGHT: u16 = 3;
const BUTTONS_ROW_HEIGHT: u16 = 3;

const INTRO_TEXT: &str = "Portfolio";

pub struct Screen {
    command_tx: mpsc::Sender<AppCommand>,
    account: Account,

    quit_button: buttons::Button,
    manage_button: buttons::MenuButton,
}

impl Screen {
    pub fn new(command_tx: mpsc::Sender<AppCommand>, account: Account) -> Self {
        let quit_button = buttons::Button::new("Quit", Some('q'));
        let manage_button = buttons::MenuButton::new(
            "Manage", Some('m'),
            vec![("Seed phrase", Some('s')), ("Delete Account", Some('d'))],
        );

        Self {
            command_tx,
            account,
            quit_button,
            manage_button,
        }
    }
}

impl AppScreen for Screen {
    fn handle_event(&mut self, event: Event) -> anyhow::Result<()> {

        if let Some(index) = self.manage_button.handle_event(&event) {
            match index {
                // 0 => {
                //     self.command_tx.send(AppCommand::SwitchScreen(Box::new(
                //         super::manage_seed_phrase::Screen::new(self.command_tx.clone(), self.account.clone())
                //     ))).unwrap();
                // },
                1 => {
                    self.command_tx.send(AppCommand::SwitchScreen(Box::new(
                        super::delete_account::Screen::new(
                            self.command_tx.clone(), self.account.clone())
                    ))).unwrap();
                },
                _ => {}
            }
        }

        if let Some(()) = self.quit_button.handle_event(&event) {
            self.command_tx.send(AppCommand::Quit).unwrap();
            return Ok(());
        }

        return Ok(());
    }

    fn render(&mut self, frame: &mut Frame) {
        let area = frame.area();
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
                Constraint::Length(INTRO_HEIGHT),
                Constraint::Min(0), // Fill height
                Constraint::Length(ACCOUNT_HEIGHT),
                Constraint::Min(0), // Fill height
                Constraint::Length(BUTTONS_ROW_HEIGHT),
                Constraint::Min(0), // Fill height
            ])
            .split(centered_area);

        let intro_text = Paragraph::new(INTRO_TEXT)
            .style(Style::default().fg(Color::Yellow).bold())
            .alignment(Alignment::Center);
        frame.render_widget(intro_text, content_layout[1]);

        // TODO: Replace with account widget
        let account_text = Paragraph::new(self.account.address.to_string())
            .style(Style::default().fg(Color::Yellow).bold())
            .alignment(Alignment::Center);
        frame.render_widget(account_text, content_layout[3]);

        let buttons_row = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ])
            .split(content_layout[5]);

        self.quit_button.render(frame, buttons_row[0]);
        self.manage_button.render(frame, buttons_row[1]);
    }
}
