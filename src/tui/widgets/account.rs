use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame
};

use crate::service::crypto::ChainBalances;

pub struct Account {
    pub name: String,
    pub address: web3::types::Address,
    pub balances: Option<ChainBalances>,
}

impl Account {
    pub fn new(address: web3::types::Address) -> Self {
        Self {
            name: "Master Keypair".to_string(), // TODO: account name
            address,
            balances: None,
        }
    }

    pub fn get_total_balances(&self) -> Option<(f64, f64, bool)> {
        match &self.balances {
            Some(balances) => {
                let mut total_value = 0.0;
                let mut total_usd = 0.0;
                let mut from_test_network = false;
                for balance in balances.values() {
                    total_value += balance.value;
                    total_usd += balance.usd_value;
                    from_test_network = balance.from_test_network || from_test_network;
                }
                Some((total_value, total_usd, from_test_network))
            }
            None => {
                None
            }
        }
    }

    fn get_account_str(&self) -> String {
        format!("ETH ({})", self.address.to_string()) // TODO: different blockchain
    }

    fn get_account_balance_str(&self) -> (String, bool) {
        if let Some((total_value, total_usd, from_test_network)) = self.get_total_balances() {
            (format!("{:.6} ETH ({:.2} USD)", total_value, total_usd), from_test_network)
        } else {
            ("Loading...".to_string(), false)
        }
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Yellow))
            .title(self.name.clone());
        let inner_area = block.inner(area);

        frame.render_widget(block, area);

        let internals = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(1),
                Constraint::Fill(1),
                Constraint::Fill(1),
                Constraint::Length(1),
            ])
            .split(inner_area);

        let eth_label = Paragraph::new(self.get_account_str())
            .style(Style::default().fg(Color::Yellow).add_modifier(ratatui::style::Modifier::BOLD))
            .alignment(Alignment::Left);

        let (balance_str, from_test_network) = self.get_account_balance_str();
        let balance_color = if from_test_network { Color::Red } else { Color::Yellow };
        let balances_label = Paragraph::new(balance_str)
            .style(Style::default().fg(balance_color).add_modifier(ratatui::style::Modifier::BOLD))
            .alignment(Alignment::Right);

        frame.render_widget(eth_label, internals[1]);
        frame.render_widget(balances_label, internals[2]);
    }
}
