use ratatui::{
    crossterm::event::Event,
    layout::{Alignment, Constraint, Direction, Layout, Margin, Rect},
    style::{Color, Style},
    widgets::Paragraph, Frame
};

use crate::service::session::Session;
use crate::tui::{widgets::{controls, transaction}, app::AppScreen};

const TITLE_HEIGHT: u16 = 2;
const TRANSACTIONAS_PER_PAGE: usize = 10;

const TITLE_TEXT: &str = "Transactions";

pub struct Page {
    session: Session,
    update: bool,
    cursor: usize,

    transactions: Vec<transaction::TransactionDisplay>,
    scroll: controls::Scroll,
}

impl Page {
    pub fn new(session: Session) -> Self {
        let transactions = Vec::new();

        let scroll = controls::Scroll::new();

        Self {
            session,
            update: true,
            cursor: 0,
            transactions,
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

        let transactions = self.session.db.get_transactions(self.session.account, self.cursor, TRANSACTIONAS_PER_PAGE)
            .unwrap_or_else(|err| {
                log::error!("Failed to fetch transactions: {:?}", err);
                Vec::new() // Empty transactions on error
            }
        );

        self.transactions = transactions.into_iter().map(|tx| {
            transaction::TransactionDisplay::new(tx, transaction::TransactionDisplayType::Incoming)
        }).collect();

        self.update = false;
    }

    fn render(&mut self, frame: &mut Frame, area: Rect) {
        let content_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(TITLE_HEIGHT),
                Constraint::Fill(0),    // Fill height for trasnactions
            ])
            .split(area);

        let title = Paragraph::new(TITLE_TEXT)
            .style(Style::default().fg(Color::Yellow).add_modifier(ratatui::style::Modifier::BOLD))
            .alignment(Alignment::Center);
        frame.render_widget(title, content_layout[0]);

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
