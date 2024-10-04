use std::sync::Arc;
use tokio::sync::Mutex;
use ratatui::{
    crossterm::event::Event,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame
};
use web3::types::Address;

use crate::service::crypto::Crypto;
use crate::tui::{widgets::controls, app::AppScreen};

const TITLE: &str = "Send Crypto";

pub struct Popup {
    crypto: Arc<Mutex<Crypto>>,

    from: web3::types::Address,
    to: Option<web3::types::Address>,
    amount: f64,

    back_button: controls::Button,
    send_button: controls::Button
}

impl Popup {
    pub fn new(from: Address, crypto: Arc<Mutex<Crypto>>) -> Self {
        let to = None;
        let amount = 0.0;

        let back_button = controls::Button::new("Back", Some('b')).escape();
        let send_button = controls::Button::new("Send Transaction", Some('s'));

        Self {
            crypto,
            from,
            to,
            amount,
            back_button,
            send_button
        }
    }

}

#[async_trait::async_trait]
impl AppScreen for Popup {
    async fn handle_event(&mut self, event: Event) -> anyhow::Result<bool> {
        if let Some(()) = self.back_button.handle_event(&event) {
            return Ok(true);
        }
        if let Some(()) = self.send_button.handle_event(&event) {
            // TODO: Send transaction
            return Ok(false);
        }
        Ok(false)
    }

    async fn update(&mut self) {}

    fn render(&mut self, frame: &mut Frame, area: Rect) {
        frame.render_widget(Clear, area);

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Yellow))
            .title(TITLE);
        let inner_area = block.inner(area);
        frame.render_widget(block, area);

        let content_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1), // Margin
                Constraint::Length(controls::BUTTON_HEIGHT), // From
                Constraint::Min(0), // Fill height
                Constraint::Length(controls::BUTTON_HEIGHT), // To
                Constraint::Min(0), // Fill height
                Constraint::Length(controls::INPUT_HEIGHT), // Amount
                Constraint::Fill(0), // Fill height
                Constraint::Length(controls::BUTTON_HEIGHT),
            ])
            .split(inner_area);

        let from_text = Paragraph::new(format!("From: {}", self.from))
            .style(Style::default().fg(Color::Yellow))
            .alignment(Alignment::Center);
        frame.render_widget(from_text, content_layout[1]);

        let to_text = Paragraph::new(format!("To: {}", self.to.unwrap_or(Address::zero())))
            .style(Style::default().fg(Color::Yellow))
            .alignment(Alignment::Center);
        frame.render_widget(to_text, content_layout[3]);


        let amount_text = Paragraph::new(format!("Amount: {}", self.amount))
            .style(Style::default().fg(Color::Yellow))
            .alignment(Alignment::Center);
        frame.render_widget(amount_text, content_layout[5]);

        let buttons_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(30),
                Constraint::Percentage(70),
            ])
            .split(content_layout[7]);

        self.back_button.render(frame, buttons_layout[0]);
        self.send_button.render(frame, buttons_layout[1]);
    }
}
