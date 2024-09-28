use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame
};

use crate::core::provider::Balance;

pub struct Account {
    pub name: String,
    pub address: web3::types::Address,
    pub balance: Option<Balance>,
}

impl Account {
    pub fn new(address: web3::types::Address) -> Self {
        Self {
            name: "Master Account".to_string(), // TODO: account name
            address,
            balance: None,
        }
    }

    fn get_account_str(&self) -> String {
        format!("ETH ({})", self.address.to_string())
    }

    fn get_balance_str(&self) -> String {
        self.balance.as_ref().map(|b| b.to_string()).unwrap_or_else(|| "Loading...".to_string())
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

        let balances_label = Paragraph::new(self.get_balance_str())
            .style(Style::default().fg(Color::Yellow).add_modifier(ratatui::style::Modifier::BOLD))
            .alignment(Alignment::Right);

        frame.render_widget(eth_label, internals[1]);
        frame.render_widget(balances_label, internals[2]);
    }
}
