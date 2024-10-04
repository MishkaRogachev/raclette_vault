use std::sync::Arc;
use tokio::sync::Mutex;
use ratatui::{
    crossterm::event::Event,
    layout::{Alignment, Constraint, Direction, Layout, Margin, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame
};
use web3::types::Address;

use crate::{core::eth_utils, service::crypto::Crypto};
use crate::tui::{widgets::controls, app::AppScreen};

const TITLE: &str = "Send Crypto";

pub struct Popup {
    crypto: Arc<Mutex<Crypto>>,

    from: web3::types::Address,
    eth_usd_rate: Option<f64>,
    fees: Option<f64>,

    to: controls::Input,
    amount: controls::Input,
    swap_button: controls::SwapButton,
    busy: controls::Busy,
    back_button: controls::Button,
    send_button: controls::Button
}

impl Popup {
    pub fn new(from: Address, crypto: Arc<Mutex<Crypto>>) -> Self {
        let eth_usd_rate = None;
        let fees = None;

        let to = controls::Input::new("Enter receiver address")
            .with_regex(regex::Regex::new(r"^0(x[0-9a-fA-F]*)?$").unwrap());
        let amount = controls::Input::new("Enter amount ETH to transfer")
            .with_regex(regex::Regex::new(r"^(0(\.\d*)?|[1-9]\d*(\.\d*)?)?$").unwrap());
        let swap_button = controls::SwapButton::new(
            controls::Button::new("ETH", Some('m')),
            controls::Button::new("USD", Some('e'))
        );
        let busy = controls::Busy::new("Loading..");
        let back_button = controls::Button::new("Back", Some('b')).escape();
        let send_button = controls::Button::new("Send Transaction", Some('s'));

        Self {
            crypto,
            from,
            eth_usd_rate,
            fees,
            to,
            amount,
            swap_button,
            busy,
            back_button,
            send_button
        }
    }

}

#[async_trait::async_trait]
impl AppScreen for Popup {
    async fn handle_event(&mut self, event: Event) -> anyhow::Result<bool> {
        if let Some(_) = controls::handle_scoped_event(&mut [&mut self.to, &mut self.amount], &event) {
            return Ok(false);
        }
        if let Some(_) = self.swap_button.handle_event(&event) {
            return Ok(false);
        }
        if let Some(()) = self.back_button.handle_event(&event) {
            return Ok(true);
        }
        if let Some(()) = self.send_button.handle_event(&event) {
            // TODO: Send transaction
            return Ok(false);
        }
        Ok(false)
    }

    async fn update(&mut self) {
        // Validate receiver address
        let to = eth_utils::str_to_eth_address(&self.to.value);
        let is_valid = to.is_ok();
        self.to.color = if is_valid { Color::Yellow } else { Color::Red };

        self.send_button.disabled = !is_valid;
    }

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
                Constraint::Length(controls::BUTTON_HEIGHT),    // Currency
                Constraint::Length(controls::BUTTON_HEIGHT),    // From
                Constraint::Length(controls::BUTTON_HEIGHT),    // To
                Constraint::Length(controls::INPUT_HEIGHT),     // Amount
                Constraint::Fill(controls::BUTTON_HEIGHT),      // Fees
                Constraint::Length(controls::BUTTON_HEIGHT),    // Buttons
            ])
            .split(inner_area);

        let row_constraints = [
            Constraint::Percentage(2),
            Constraint::Percentage(21),
            Constraint::Percentage(75),
            Constraint::Percentage(2),
        ];

        let label_margin = Margin { vertical: 1, horizontal: 1 };

        // Currency
        let currency_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(row_constraints)
            .split(content_layout[0]);

        let currency_label = Paragraph::new("Currency")
            .style(Style::default().fg(Color::Yellow))
            .alignment(Alignment::Left);
        frame.render_widget(currency_label, currency_layout[1].inner(label_margin));

        let currency = Paragraph::new("ETH")
            .style(Style::default().fg(Color::Yellow))
            .alignment(Alignment::Left);
        frame.render_widget(currency, currency_layout[2].inner(label_margin));

        // From
        let from_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(row_constraints)
            .split(content_layout[1]);

        let from_label = Paragraph::new("From")
            .style(Style::default().fg(Color::Yellow))
            .alignment(Alignment::Left);
        frame.render_widget(from_label, from_layout[1].inner(label_margin));

        let from_address = Paragraph::new(
            format!("Master Keypair ({})", self.from.to_string()))
            .style(Style::default().fg(Color::Yellow))
            .alignment(Alignment::Left);
        frame.render_widget(from_address, from_layout[2].inner(label_margin));

        let to_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(row_constraints)
            .split(content_layout[2]);

        let to_label = Paragraph::new("To")
            .style(Style::default().fg(Color::Yellow))
            .alignment(Alignment::Left);
        frame.render_widget(to_label, to_layout[1].inner(label_margin));

        self.to.render(frame, to_layout[2]);

        // Amount
        let amount_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Percentage(2),
                Constraint::Percentage(21),     // Label
                Constraint::Percentage(35),     // Input
                Constraint::Percentage(15),     // Swap button
                Constraint::Percentage(25),     // Alt value
                Constraint::Percentage(2)])
            .split(content_layout[3]);

        let amount_label = Paragraph::new("Amount")
            .style(Style::default().fg(Color::Yellow))
            .alignment(Alignment::Left);
        frame.render_widget(amount_label, amount_layout[1].inner(label_margin));

        self.amount.render(frame, amount_layout[2]);
        self.swap_button.render(frame, amount_layout[3]);

        let amount_value = self.amount.value.parse::<f64>().unwrap_or(0.0);
        let alt_amount = if self.swap_button.state {
            if let Some(eth_usd_rate) = self.eth_usd_rate {
                Some(format!("{} ETH", eth_usd_rate * amount_value))
            } else {
                None
            }
        } else {
            if let Some(eth_usd_rate) = self.eth_usd_rate {
                Some(format!("{} USD", eth_usd_rate * amount_value))
            } else {
                None
            }
        };
        let alt_layout = amount_layout[4].inner(label_margin);
        if let Some(alt_amount) = alt_amount {
            let alt_amount_label = Paragraph::new(alt_amount)
                .style(Style::default().fg(Color::Yellow))
                .alignment(Alignment::Left);
            frame.render_widget(alt_amount_label, alt_layout);
        } else if amount_value > 0.0 {
            self.busy.render(frame, alt_layout);
        }

        // Fees
        let fees_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(row_constraints)
            .split(content_layout[4]);

        let fees_label = Paragraph::new("Fees")
            .style(Style::default().fg(Color::Yellow))
            .alignment(Alignment::Left);
        frame.render_widget(fees_label, fees_layout[1].inner(label_margin));

        let fees_value = if let Some(fees) = self.fees {
            Paragraph::new(format!("{:.6} ETH", fees))
                .style(Style::default().fg(Color::Yellow))
                .alignment(Alignment::Left)
        } else {
            Paragraph::new("-")
                .style(Style::default().fg(Color::Yellow))
                .alignment(Alignment::Left)
        };
        frame.render_widget(fees_value, fees_layout[2].inner(label_margin));

        // Buttons
        let buttons_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(30),
                Constraint::Percentage(70),
            ])
            .split(content_layout[5]);

        self.back_button.render(frame, buttons_layout[0]);
        self.send_button.render(frame, buttons_layout[1]);
    }
}
