use ratatui::{
    layout::{Alignment, Margin, Rect},
    style::{Color, Style},
    widgets::Paragraph,
    Frame
};

use crate::core::transaction::TransactionResult;

const TRANSACTION_HEIGHT: usize = 3;

pub enum TransactionDisplayType {
    Incoming,
    Outgoing,
    Swap,
}

pub struct TransactionDisplay {
    transaction: TransactionResult,
    transaction_type: TransactionDisplayType,
}

impl TransactionDisplay {
    pub fn new(transaction: TransactionResult, transaction_type: TransactionDisplayType) -> Self {
        Self {
            transaction,
            transaction_type,
        }
    }

    pub fn implicit_height(&self) -> usize {
        TRANSACTION_HEIGHT
    }

    pub fn get_transaction_str(&self) -> String {
        let amount = self.transaction.amount;
        let currency = "ETH"; // TODO: different blockchains & tokens
        let from = self.transaction.from.unwrap_or_default();
        let to = self.transaction.to.unwrap_or_default();

        match self.transaction_type {
            TransactionDisplayType::Incoming => 
                format!("↓ Received {} {} from {}", amount, currency, from),
            TransactionDisplayType::Outgoing => 
                format!("↑ Sent {} {} to {}", amount, currency, to),
            TransactionDisplayType::Swap => 
                format!("↕ Swap {} {} from {} to {}", amount, currency, from, to),
        }
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        let inner = area.inner(Margin { vertical: 1, horizontal: 1 });

        let paragraph = Paragraph::new(self.get_transaction_str())
            .style(Style::default().fg(Color::Yellow))
            .alignment(Alignment::Left);
    }
}
