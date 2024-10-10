use std::sync::Arc;
use tokio::sync::Mutex;
use ratatui::{
    crossterm::event::Event,
    layout::{Alignment, Constraint, Direction, Layout, Margin, Rect},
    style::{Color, Style},
    widgets::Paragraph, Frame
};

use crate::service::{crypto:: Crypto, session::Session};
use crate::tui::{widgets::{controls, transaction}, app::AppScreen};

const TITLE_HEIGHT: u16 = 2;
const TITLE_TEXT: &str = "Transaction history:";

const TRANSACTION_CNT_PER_PAGE: usize = 10;

pub struct Page {
    session: Session,
    crypto: Arc<Mutex<Crypto>>,
    update: bool,

    transactions: Vec<transaction::TransactionDisplay>,
    busy: controls::Busy,
    scroll: controls::Scroll,
}

impl Page {
    pub fn new(session: Session, crypto: Arc<Mutex<Crypto>>) -> Self {
        let transactions = Vec::new();

        let busy = controls::Busy::new("Loading..");
        let scroll = controls::Scroll::new();

        Self {
            session,
            crypto,
            update: true,
            transactions,
            busy,
            scroll
        }
    }
}

#[async_trait::async_trait]
impl AppScreen for Page {
    async fn handle_event(&mut self, event: Event) -> anyhow::Result<bool> {
        self.scroll.handle_event(&event);
        Ok(false)
    }

    async fn update(&mut self) {
        if !self.update {
            return;
        }

        let crypto = self.crypto.lock().await;

        // TODO: fetch transactions
        // = session.db.get_transactions(
        //     session.account, 0, TRANSACTION_CNT_PER_PAGE).expect(
        //         "Failed to get transactions"
        //     );

        // let transactions = transactions.into_iter().map(|tx| {
        //     transaction::TransactionDisplay::new(tx, transaction::TransactionDisplayType::Incoming)
        // }).collect();
        //let transactions = crypto.ge
    }

    fn render(&mut self, frame: &mut Frame, area: Rect) {
        let content_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(TITLE_HEIGHT),
                Constraint::Fill(0),    // Fill height for trasnactions
            ])
            .split(area);

        let summary = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(35),
                Constraint::Percentage(65),
            ])
            .split(content_layout[0]);

        let summary_label = Paragraph::new(TITLE_TEXT)
            .style(Style::default().fg(Color::Yellow).add_modifier(ratatui::style::Modifier::BOLD))
            .alignment(Alignment::Left);
        frame.render_widget(summary_label, summary[0]);

        let transactions_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(self.transactions.iter().map(|_| Constraint::Fill(1)).collect::<Vec<_>>().as_slice())
        .split(content_layout[1].inner(Margin {
            vertical: 0,
            horizontal: 1,
        }));

        let mut total_content_height = 0;
        for (transaction, transaction_layout) in self.transactions
            .iter_mut().zip(transactions_layout.iter()) {
            //transaction.scroll_offset = self.scroll.position;
            transaction.render(frame, *transaction_layout);
            total_content_height += transaction.implicit_height();
        }

        self.scroll.total = total_content_height;
        self.scroll.render(frame, content_layout[1]);
    }
}

impl super::porfolio::PorfolioPage for Page {
    fn on_networks_change(&mut self) {
        self.update = true;
    }
}
