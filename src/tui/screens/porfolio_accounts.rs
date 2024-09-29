use std::sync::Arc;
use tokio::sync::Mutex;
use ratatui::{
    crossterm::event::Event,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::Paragraph, Frame
};

use crate::service::{crypto:: Crypto, session::Session};
use crate::tui::{widgets::account, app::AppScreen};

const SUMMARY_HEIGHT: u16 = 2;
const SUMMARY_TEXT: &str = "Summary balance";

pub struct Screen {
    session: Session,
    crypto: Arc<Mutex<Crypto>>,
    account: account::Account,
}

impl Screen {
    pub fn new(session: Session, crypto: Arc<Mutex<Crypto>>) -> Self {

        let account = account::Account::new(session.account);

        Self {
            session,
            crypto,
            account,
        }
    }

    fn get_summary_balance_str(&self) -> (String, bool) {
        if let Some(balance) = &self.account.balance {
            let end = format!("{:.2} USD", balance.usd_value);
            if balance.from_test_network {
                (format!("{} (testnet)", end), true)
            } else {
                (end, false)
            }
        } else {
            ("Loading...".to_string(), false)
        }
    }
}

#[async_trait::async_trait]
impl AppScreen for Screen {
    fn handle_event(&mut self, _event: Event) -> anyhow::Result<bool> {
        return Ok(false);
    }

    async fn update(&mut self) {
        let crypto = self.crypto.lock().await;
        if self.account.balance.is_none() {
            let balance = crypto.get_eth_balance(self.session.account)
                .await
                .expect("Failed to get balance");
            self.account.balance = Some(balance);
        }
    }

    fn render(&mut self, frame: &mut Frame, area: Rect) {
        let content_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(SUMMARY_HEIGHT),
                Constraint::Fill(0), // Fill height for accounts
            ])
            .split(area);

        let summary = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(35),
                Constraint::Percentage(65),
            ])
            .split(content_layout[0]);

        let summary_label = Paragraph::new(SUMMARY_TEXT)
            .style(Style::default().fg(Color::Yellow).add_modifier(ratatui::style::Modifier::BOLD))
            .alignment(Alignment::Left);

        let (summary_balance, from_test_network) = self.get_summary_balance_str();
        let balance_color = if from_test_network { Color::Red } else { Color::Yellow };
        let balances_label = Paragraph::new(summary_balance)
            .style(Style::default().fg(balance_color).add_modifier(ratatui::style::Modifier::BOLD))
            .alignment(Alignment::Right);

        frame.render_widget(summary_label, summary[0]);
        frame.render_widget(balances_label, summary[1]);

        // TODO: Several accounts
        self.account.render(frame, content_layout[1]);
    }
}
