use std::sync::Arc;
use tokio::sync::Mutex;
use ratatui::{
    crossterm::event::Event,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Paragraph, Widget}, Frame
};

use crate::service::{crypto:: Crypto, session::Session};
use crate::tui::{widgets::{controls, account}, app::AppScreen};

const SUMMARY_HEIGHT: u16 = 2;
const SUMMARY_TEXT: &str = "Summary balance";

const UPDATE_INTERVAL: tokio::time::Duration = tokio::time::Duration::from_secs(10);

pub struct Screen {
    crypto: Arc<Mutex<Crypto>>,
    last_update: Option<tokio::time::Instant>,

    accounts: Vec<account::Account>,
    busy: controls::Busy,
}

impl Screen {
    pub fn new(session: Session, crypto: Arc<Mutex<Crypto>>) -> Self {
        let accounts = vec![account::Account::new(session.account)];
        let busy = controls::Busy::new("Loading..");

        Self {
            crypto,
            last_update: None,
            accounts,
            busy,
        }
    }

    fn render_summary_balance_str(&mut self, frame: &mut Frame, area: Rect) {
        let mut usd_summary = None;
        let mut test_network = false;
        for account in &self.accounts {
            if let Some((usd_value, from_test_network)) = &account.get_total_usd_balance() {
                usd_summary = Some(usd_summary.unwrap_or(0.0) + usd_value);
                test_network = test_network || *from_test_network;
            }
        }
        match usd_summary {
            Some(usd_summary) => {
                let balances_str = format!("{} {:.2} USD", if test_network {"(Testnet) "} else { "" }, usd_summary);
                let balances_color = if test_network { Color::Red } else { Color::Yellow };
                Paragraph::new(balances_str)
                    .style(Style::default().fg(balances_color).add_modifier(ratatui::style::Modifier::BOLD))
                    .alignment(Alignment::Right)
                    .render(area, frame.buffer_mut());
            },
            None => {
                self.busy.render(frame, area);
            },
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
            account.balances = crypto.get_balances(account.address).await;
        }

        if self.last_update.is_none() || self.last_update.unwrap().elapsed() > UPDATE_INTERVAL {
            let accounts = self.accounts.iter().map(|account| account.address).collect();
            crypto.fetch_balances(accounts).await;
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
        frame.render_widget(summary_label, summary[0]);

        self.render_summary_balance_str(frame, summary[1]);

        let accounts_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(self.accounts.iter().map(|_| Constraint::Fill(1)).collect::<Vec<_>>().as_slice())
            .split(content_layout[1]);
        for (account, account_layout) in self.accounts.iter_mut().zip(accounts_layout.iter()) {
            account.render(frame, *account_layout);
        }
    }
}

