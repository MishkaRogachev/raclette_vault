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

const UPDATE_INTERVAL: tokio::time::Duration = tokio::time::Duration::from_millis(2000);

pub struct Screen {
    crypto: Arc<Mutex<Crypto>>,
    last_update: Option<tokio::time::Instant>,

    accounts: Vec<account::Account>,
}

impl Screen {
    pub fn new(session: Session, crypto: Arc<Mutex<Crypto>>) -> Self {
        let accounts = vec![account::Account::new(session.account)];

        Self {
            crypto,
            last_update: None,
            accounts,
        }
    }

    fn get_summary_balance_str(&self) -> (String, bool) {
        let mut usd_summary = None;
        let mut test_network = false;
        for account in &self.accounts {
            if let Some((_, usd_value, from_test_network)) = &account.get_total_balances() {
                usd_summary = Some(usd_summary.unwrap_or(0.0) + usd_value);
                test_network = test_network || *from_test_network;
            }
        }
        match usd_summary {
            Some(usd_summary) => (
                format!("{} {:.2} USD", if test_network {"(Testnet) "} else { "" }, usd_summary),
                test_network
            ),
            None => (String::from("Loading.."), false),
        }
    }
}

#[async_trait::async_trait]
impl AppScreen for Screen {
    async fn handle_event(&mut self, _event: Event) -> anyhow::Result<bool> {
        return Ok(false);
    }

    async fn update(&mut self) {
        let mut crypto = self.crypto.lock().await;

        for account in &mut self.accounts {
            account.balances = crypto.get_eth_balances(account.address).await;
        }

        if self.last_update.is_none() || self.last_update.unwrap().elapsed() > UPDATE_INTERVAL {
            let accounts = self.accounts.iter().map(|account| account.address).collect();
            crypto.fetch_eth_balances(accounts).await;
            self.last_update = Some(tokio::time::Instant::now());
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

        let accounts_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(self.accounts.iter().map(|_| Constraint::Length(3)).collect::<Vec<_>>().as_slice())
            .split(content_layout[1]);
        for (account, account_layout) in self.accounts.iter_mut().zip(accounts_layout.iter()) {
            account.render(frame, *account_layout);
        }
    }
}

