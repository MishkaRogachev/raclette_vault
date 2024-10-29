use copypasta::{ClipboardContext, ClipboardProvider};
use qrcode::render::unicode;
use ratatui::{
    crossterm::event::Event,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame
};

use crate::tui::{widgets::controls, app::AppScreen};

const TITLE: &str = "Receive Crypto";

pub struct Popup {
    address: web3::types::Address,
    back_button: controls::Button,
    copy_button: controls::Button,
    copied: bool,
}

impl Popup {
    pub fn new(address: web3::types::Address) -> Self {
        let back_button = controls::Button::new("Back", Some('b')).escape();
        let copy_button = controls::Button::new("Copy To Clipboard", Some('c'));

        Self {
            address,
            back_button,
            copy_button,
            copied: false,
        }
    }

    fn full_address(&self) -> String {
        format!("0x{}", hex::encode(self.address.as_bytes()))
    }

    fn generate_qr_code(&self) -> String {
        let qr_code = qrcode::QrCode::new(self.full_address()).unwrap();
        qr_code
            .render::<unicode::Dense1x2>()  // Use dense Unicode characters
            .dark_color(unicode::Dense1x2::Dark)
            .light_color(unicode::Dense1x2::Light)
            .build()
    }
}

#[async_trait::async_trait]
impl AppScreen for Popup {
    async fn handle_event(&mut self, event: Event) -> anyhow::Result<bool> {
        if let Some(()) = self.back_button.handle_event(&event) {
            return Ok(true);
        }
        if let Some(()) = self.copy_button.handle_event(&event) {
            let mut ctx = ClipboardContext::new().unwrap();
            ctx.set_contents(self.full_address()).unwrap();
            self.copied = true;
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
                Constraint::Length(1),  // margin
                Constraint::Length(1),  // Address
                Constraint::Length(1),  // Copied
                Constraint::Fill(0),    // QR code
                Constraint::Length(controls::BUTTON_HEIGHT),
            ])
            .split(inner_area);

        let address_paragraph = Paragraph::new(format!("Address: {}", self.full_address()))
            .style(Style::default().fg(Color::Yellow))
            .alignment(Alignment::Center);
        frame.render_widget(address_paragraph, content_layout[1]);

        if self.copied {
            let copied_paragraph = Paragraph::new("Copied!")
                .style(Style::default().fg(Color::Yellow))
                .alignment(Alignment::Center);
            frame.render_widget(copied_paragraph, content_layout[2]);
        }

        let qr_code_string = self.generate_qr_code();
        let qr_code_paragraph = Paragraph::new(qr_code_string)
            .style(Style::default().fg(Color::Yellow))
            .alignment(Alignment::Center);
        frame.render_widget(qr_code_paragraph, content_layout[3]);

        let buttons_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(30),
                Constraint::Percentage(70),
            ])
            .split(content_layout[4]);

        self.back_button.render(frame, buttons_layout[0]);
        self.copy_button.render(frame, buttons_layout[1]);
    }
}
