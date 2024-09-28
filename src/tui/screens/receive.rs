use copypasta::{ClipboardContext, ClipboardProvider};
use ratatui::{
    crossterm::event::Event,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame
};

use crate::tui::{widgets::buttons, app::AppScreen};

const RECEIVE_WIDTH: u16 = 60;
const V_PADDING: u16 = 2;
const ACCOUNT_HEIGHT: u16 = 2;
const BUTTONS_ROW_HEIGHT: u16 = 3;

const TITLE: &str = "Receive Crypto";

pub struct Screen {
    address: web3::types::Address,
    back_button: buttons::Button,
    copy_button: buttons::Button
}

impl Screen {
    pub fn new(address: web3::types::Address) -> Self {
        let back_button = buttons::Button::new("Back", Some('b'));
        let copy_button = buttons::Button::new("Copy To Clipboard", Some('c'));

        Self {
            address,
            back_button,
            copy_button
        }
    }

    fn full_address(&self) -> String {
        format!("0x{}", hex::encode(self.address.as_bytes()))
    }
}

#[async_trait::async_trait]
impl AppScreen for Screen {
    fn handle_event(&mut self, event: Event) -> anyhow::Result<bool> {
        if let Some(()) = self.back_button.handle_event(&event) {
            return Ok(true);
        }
        if let Some(()) = self.copy_button.handle_event(&event) {
            let mut ctx = ClipboardContext::new().unwrap();
            ctx.set_contents(self.full_address()).unwrap();
        }
        Ok(false)
    }

    async fn update(&mut self) {}

    fn render(&mut self, frame: &mut Frame) {
        let area = frame.area();
        let horizontal_padding = (area.width.saturating_sub(RECEIVE_WIDTH)) / 2;

        let popup_area = Rect {
            x: horizontal_padding,
            y: area.y + V_PADDING,
            width: RECEIVE_WIDTH,
            height: area.height - V_PADDING * 2,
        };
        frame.render_widget(Clear, popup_area);

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Yellow))
            .title(TITLE);
        let inner_area = block.inner(popup_area);
        frame.render_widget(block, popup_area);

        let content_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(ACCOUNT_HEIGHT),
                Constraint::Fill(0), // Fill height for QR code
                Constraint::Length(BUTTONS_ROW_HEIGHT),
            ])
            .split(inner_area);

        let address_text = format!("Address: {}", self.full_address());
        let address_paragraph = Paragraph::new(address_text)
            .style(Style::default().fg(Color::Yellow))
            .alignment(Alignment::Center);
        frame.render_widget(address_paragraph, content_layout[0]);

        // TODO: QR Code

        let buttons_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(30),
                Constraint::Percentage(70),
            ])
            .split(content_layout[2]);

        self.back_button.render(frame, buttons_layout[0]);
        self.copy_button.render(frame, buttons_layout[1]);
    }
}
