use std::sync::{atomic::AtomicBool, mpsc, Arc};

use ratatui::{
    crossterm::event::Event,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    widgets::Paragraph, Frame
};

use crate::service::account::Account;
use crate::tui::{widgets::{buttons, common}, app::AppCommand};

const HOME_WIDTH: u16 = 60;
const INTRO_HEIGHT: u16 = 1;
const ACCOUNT_HEIGHT: u16 = 3;
const BUTTONS_ROW_HEIGHT: u16 = 3;

const INTRO_TEXT: &str = "Portfolio";

pub struct Screen {
    account: Account,

    quit_button: buttons::Button,
    reveal_button: buttons::SwapButton,
}

impl Screen {
    pub fn new(command_tx: mpsc::Sender<AppCommand>, account: Account) -> Self {
        let reveal_flag = Arc::new(AtomicBool::new(false));

        let quit_button = buttons::Button::new("Quit", Some('q'))
            .on_down(move || {
                command_tx.send(AppCommand::Quit).unwrap();
            });

        let reveal_button = buttons::SwapButton::new(
            reveal_flag, "Reveal", Some('r'), "Hide", Some('h'));

        Self {
            account,
            quit_button,
            reveal_button,
        }
    }
}

impl common::Widget for Screen {
    fn handle_event(&mut self, event: Event) -> Option<Event> {
        let mut controls: Vec<&mut dyn common::Widget> = vec![
            &mut self.quit_button,
            &mut self.reveal_button,
        ];
        controls.iter_mut().fold(Some(event), |event, button| {
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

        self.quit_button.process(frame, buttons_row[0]);
        self.reveal_button.process(frame, buttons_row[1]);
    }
}