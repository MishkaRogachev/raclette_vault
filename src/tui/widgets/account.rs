use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Widget},
    Frame
};

use crate::core::balance::Balances;

use super::controls;

pub struct Account {
    pub name: String,
    pub address: web3::types::Address,
    pub balances: Option<Balances>,
    busy: controls::Busy,
}

impl Account {
    pub fn new(address: web3::types::Address) -> Self {
        let busy = controls::Busy::new("Loading..");

        Self {
            name: "Master Keypair".to_string(), // TODO: account name
            address,
            balances: None,
            busy,
        }
    }

    pub fn get_total_balances(&self) -> Option<(f64, f64, bool)> {
        self.balances.as_ref().map(|balances| {
            balances.iter().fold((0.0, 0.0, false), |(total_value, total_usd, from_test), balance| {
                (
                    total_value + balance.value,
                    total_usd + balance.usd_value,
                    from_test || balance.from_test_network,
                )
            })
        })
    }

    fn get_account_str(&self) -> String {
        format!("ETH ({})", self.address.to_string()) // TODO: different blockchain
    }

    fn render_total_balances(&mut self, frame: &mut Frame, area: Rect) {
        if let Some((total_value, total_usd, from_test_network)) = self.get_total_balances() {
            let balances_str = format!("{:.6} ETH ({:.2} USD)", total_value, total_usd);
            let balances_color = if from_test_network { Color::Red } else { Color::Yellow };
            Paragraph::new(balances_str)
                .style(Style::default().fg(balances_color).add_modifier(ratatui::style::Modifier::BOLD))
                .alignment(Alignment::Right)
                .render(area, frame.buffer_mut());
        } else {
            self.busy.render(frame, area);
        }
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Yellow))
            .title(self.name.clone());
        let inner_area = block.inner(area);
        frame.render_widget(block, area);

        let balances_cnt = if let Some(balances) = &self.balances { balances.len() } else { 0 };
        let tokens_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints((0..balances_cnt + 1).map(|_| Constraint::Length(1)).collect::<Vec<_>>())
            .split(inner_area);

        let header_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(1),
                Constraint::Fill(1),
                Constraint::Fill(1),
                Constraint::Length(1),
            ])
            .split(tokens_layout[0]);

        let eth_label = Paragraph::new(self.get_account_str())
            .style(Style::default().fg(Color::Yellow).add_modifier(ratatui::style::Modifier::BOLD))
            .alignment(Alignment::Left);
        frame.render_widget(eth_label, header_layout[1]);

        self.render_total_balances(frame, header_layout[2]);

        for i in 0..balances_cnt {
            let token_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Length(2),
                    Constraint::Fill(1),
                    Constraint::Fill(1),
                    Constraint::Length(1),
                ])
                .split(tokens_layout[i + 1]);

            let token = &self.balances.as_ref().unwrap()[i];
            let token_label = Paragraph::new(format!("{}", token.currency))
                .style(Style::default().fg(Color::Yellow))
                .alignment(Alignment::Left);
            // TODO: precision from token decimals
            let token_value_label = Paragraph::new(format!("{:.6} ({:.2} USD)", token.value, token.usd_value))
                .style(Style::default().fg(Color::Yellow))
                .alignment(Alignment::Right);

            frame.render_widget(token_label, token_layout[1]);
            frame.render_widget(token_value_label, token_layout[2]);
        }
    }
}
