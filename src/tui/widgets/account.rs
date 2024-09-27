use web3::types::H160;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame
};

use crate::core::provider::Balance;

pub struct Account {
    pub address: H160,
    pub balance: Option<Balance>,
}

impl Account {
    pub fn new(address: H160) -> Self {
        Self {
            address,
            balance: None,
        }
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        let balance = self.balance.as_ref().map(|b| b.to_string()).unwrap_or_else(|| "Loading...".to_string());

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Yellow))
            .title(self.address.to_string());
        let inner_area = block.inner(area);

        frame.render_widget(block, area);

        let columns = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(20),
                Constraint::Percentage(80),
            ])
            .split(inner_area);

        let eth_label = Paragraph::new("ETH")
            .style(Style::default().fg(Color::Yellow).add_modifier(ratatui::style::Modifier::BOLD))
            .alignment(Alignment::Left);

        let balances_label = Paragraph::new(balance)
            .style(Style::default().fg(Color::Yellow).add_modifier(ratatui::style::Modifier::BOLD))
            .alignment(Alignment::Right);

        frame.render_widget(eth_label, columns[0]);
        frame.render_widget(balances_label, columns[1]);
    }
}
