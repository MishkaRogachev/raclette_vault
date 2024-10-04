use std::sync::Arc;
use tokio::sync::Mutex;
use web3::types::Address;
use ratatui::{
    crossterm::event::Event,
    layout::{Alignment, Constraint, Direction, Layout, Margin, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame
};

use crate::core::{balance::Balance, eth_chain::EthChain, eth_utils, transaction::TransactionRequest};
use crate::service::crypto::Crypto;
use crate::tui::{widgets::controls, app::AppScreen};

const TITLE: &str = "Send Crypto";

pub struct Popup {
    crypto: Arc<Mutex<Crypto>>,

    chain: Option<EthChain>,
    from: web3::types::Address,
    eth_usd_rate: Option<f64>,
    amount_value: f64,
    alt_amount_value: Option<f64>,
    fees: Option<Balance>,

    chain_button: controls::MenuButton<EthChain>,
    to: controls::Input,
    amount: controls::Input,
    swap_button: controls::SwapButton,
    busy: controls::Busy,
    back_button: controls::Button,
    send_button: controls::Button
}

impl Popup {
    pub async fn new(from: Address, crypto: Arc<Mutex<Crypto>>) -> Self {
        let chain = None;
        let eth_usd_rate = None;
        let amount_value = 0.0;
        let alt_amount_value = None;
        let fees = None;

        let crypto_lock = crypto.lock().await.clone();
        let chain_options = crypto_lock.get_active_networks().iter().map(|chain| {
            (chain.clone(), controls::Button::new(chain.get_display_name(), None))
        }).collect();

        let chain_button = controls::MenuButton::new("Chain", Some('c'), chain_options);
        let to = controls::Input::new("Enter receiver address")
            .with_regex(regex::Regex::new(r"^$|^0(x[0-9a-fA-F]*)?$").unwrap());
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
            chain,
            from,
            eth_usd_rate,
            amount_value,
            alt_amount_value,
            fees,
            chain_button,
            to,
            amount,
            swap_button,
            busy,
            back_button,
            send_button
        }
    }

    fn assembly_transaction_request(&self) -> Option<TransactionRequest> {
        let chain = self.chain?;
        let to = eth_utils::str_to_eth_address(&self.to.value).ok()?;
        if self.amount_value <= 0.0 {
            return None;
        }

        Some(TransactionRequest {
            currency: "ETH".to_string(),
            chain,
            from: self.from,
            to,
            amount: self.amount_value,
        })
    }

    fn invalidate(&mut self) {
        self.amount_value = 0.0;
        self.alt_amount_value = None;
        self.fees = None;
    }
}

#[async_trait::async_trait]
impl AppScreen for Popup {
    async fn handle_event(&mut self, event: Event) -> anyhow::Result<bool> {
        if let Some(_) = controls::handle_scoped_event(&mut [&mut self.to, &mut self.amount], &event) {
            self.invalidate();
            return Ok(false);
        }
        if let Some(chain) = self.chain_button.handle_event(&event) {
            self.chain = Some(chain);
            self.invalidate();
            return Ok(false);
        }
        if let Some(_) = self.swap_button.handle_event(&event) {
            if let Some(alt_amount_value) = self.alt_amount_value {
                self.amount_value = alt_amount_value;
            }
            self.alt_amount_value = None;
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
        let mut is_ready = true;

        // Validate chain
        if let Some(chain) = self.chain {
            self.chain_button.button.label = chain.get_display_name().to_string();
            self.chain_button.button.color = Color::Yellow;
            is_ready &= true;
        } else {
            self.chain_button.button.label = "Select chain".to_string();
            self.chain_button.button.color = Color::Red;
            is_ready &= false;
        }

        // Validate receiver address
        let to = eth_utils::str_to_eth_address(&self.to.value);
        let address_valid = to.is_ok();
        self.to.color = if address_valid || self.to.value.is_empty() { Color::Yellow } else { Color::Red };
        is_ready &= address_valid;

        // Validate amount
        self.amount_value = self.amount.value.parse::<f64>().unwrap_or(0.0);
        let amount_valid = self.amount_value > 0.0;
        self.amount.color = if amount_valid || self.amount.value.is_empty() { Color::Yellow } else { Color::Red };
        is_ready &= amount_valid;

        // Calc fees
        if self.fees.is_none() && amount_valid && address_valid {
            if let Some(transaction_request) = self.assembly_transaction_request() {
                let crypto = self.crypto.lock().await.clone();
                self.fees = crypto.estimate_transaction_fees(transaction_request).await.ok();
            } else {
                self.fees = None;
            }
        }
        is_ready &= self.fees.is_some();

        // Calc alt amount
        if amount_valid && self.alt_amount_value.is_none() {
            if let Some(eth_usd_rate) = self.eth_usd_rate {
                self.alt_amount_value = Some(if self.swap_button.state {
                    self.amount_value * eth_usd_rate
                } else {
                    self.amount_value / eth_usd_rate
                });
            } else {
                self.alt_amount_value = None;
            }
        } else {
            self.alt_amount_value = None;
        }

        self.send_button.disabled = !is_ready;
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
                Constraint::Length(controls::BUTTON_HEIGHT),    // Chain
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

        // Chain
        let chain_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(row_constraints)
            .split(content_layout[1]);

        let chain_label = Paragraph::new("Chain")
            .style(Style::default().fg(Color::Yellow))
            .alignment(Alignment::Left);
        frame.render_widget(chain_label, chain_layout[1].inner(label_margin));
        // NOTE: Chain should be rendered last to ensure it's on top

        // From
        let from_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(row_constraints)
            .split(content_layout[2]);

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
            .split(content_layout[3]);

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
            .split(content_layout[4]);

        let amount_label = Paragraph::new("Amount")
            .style(Style::default().fg(Color::Yellow))
            .alignment(Alignment::Left);
        frame.render_widget(amount_label, amount_layout[1].inner(label_margin));

        self.amount.render(frame, amount_layout[2]);
        self.swap_button.render(frame, amount_layout[3]);

        let alt_layout = amount_layout[4].inner(label_margin);
        if let Some(alt_amount_value) = self.alt_amount_value {
            let alt_amount_str = if self.swap_button.state {
                format!("{} ETH", alt_amount_value)
            } else {
                format!("{} USD", alt_amount_value)
            };
            let alt_amount_label = Paragraph::new(alt_amount_str)
                .style(Style::default().fg(Color::Yellow))
                .alignment(Alignment::Left);
            frame.render_widget(alt_amount_label, alt_layout);
        } else if self.amount_value > 0.0 {
            self.busy.render(frame, alt_layout);
        }

        // Fees
        let fees_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(row_constraints)
            .split(content_layout[5]);

        let fees_label = Paragraph::new("Fees")
            .style(Style::default().fg(Color::Yellow))
            .alignment(Alignment::Left);
        frame.render_widget(fees_label, fees_layout[1].inner(label_margin));

        let fees_value = if let Some(fees) = &self.fees {
            Paragraph::new(fees.to_string())
                .style(Style::default().fg(Color::Yellow))
                .alignment(Alignment::Left)
        } else {
            Paragraph::new("---")
                .style(Style::default().fg(Color::Yellow))
                .alignment(Alignment::Left)
        };
        frame.render_widget(fees_value, fees_layout[2].inner(label_margin));

        // Chains menu
        self.chain_button.render(frame, chain_layout[2]);

        // Buttons
        let buttons_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(30),
                Constraint::Percentage(70),
            ])
            .split(content_layout[6]);

        self.back_button.render(frame, buttons_layout[0]);
        self.send_button.render(frame, buttons_layout[1]);
    }
}
