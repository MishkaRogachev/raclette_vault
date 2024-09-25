use std::sync::mpsc;
use ratatui::{
    crossterm::event::Event,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    widgets::Paragraph, Frame
};
use web3::types::U256;

use crate::{core::{chain::Chain, provider::Provider}, service::session::Session};
use crate::tui::{widgets::buttons, app::{AppCommand, AppScreen}};

const PORFOLIO_WIDTH: u16 = 80;
const INTRO_HEIGHT: u16 = 2;
const ACCOUNT_HEIGHT: u16 = 3;
const BUTTONS_ROW_HEIGHT: u16 = 3;

const INTRO_TEXT: &str = "Portfolio";

pub struct Screen {
    command_tx: mpsc::Sender<AppCommand>,
    session: Session,
    provider: Provider,
    chain: Chain,
    balance: Option<U256>,

    quit_button: buttons::Button,
    manage_button: buttons::MenuButton,
}

impl Screen {
    pub fn new(command_tx: mpsc::Sender<AppCommand>, session: Session) -> Self {
        let infura_token = std::env::var("INFURA_TOKEN")
            .expect("INFURA_TOKEN env var is not set");
        let chain = Chain::EthereumMainnet;
        let provider = Provider::new(&infura_token, chain)
            .expect("Failed to create provider");

        let quit_button = buttons::Button::new("Quit", Some('q'));
        let mut access_mnemonic = buttons::Button::new("Access mnemonic", Some('a'));
        if session.get_seed_phrase().is_err() {
            access_mnemonic.disabled = true;
        }
        let delete_account = buttons::Button::new("Delete Account", Some('d'));
        let manage_button = buttons::MenuButton::new(
            "Manage", Some('m'), vec![access_mnemonic, delete_account]
        );

        Self {
            command_tx,
            session,
            provider,
            chain,
            balance: None,
            quit_button,
            manage_button,
        }
    }
}

#[async_trait::async_trait]
impl AppScreen for Screen {
    fn handle_event(&mut self, event: Event) -> anyhow::Result<()> {
        if let Some(index) = self.manage_button.handle_event(&event) {
            match index {
                0 => {
                    self.command_tx.send(AppCommand::SwitchScreen(Box::new(
                        super::mnemonic_access::Screen::new(self.command_tx.clone(), self.session.clone())
                    ))).unwrap();
                },
                1 => {
                    self.command_tx.send(AppCommand::SwitchScreen(Box::new(
                        super::account_delete::Screen::new(
                            self.command_tx.clone(), self.session.clone())
                    ))).unwrap();
                },
                _ => {}
            }
        }

        if let Some(()) = self.quit_button.handle_event(&event) {
            self.command_tx.send(AppCommand::Quit).unwrap();
            return Ok(());
        }

        return Ok(());
    }

    async fn update(&mut self) {
        if self.balance.is_none() {
            let balance = self.provider.get_eth_balance(self.session.account)
                .await
                .expect("Failed to get balance");
            self.balance = Some(balance);
        }
    }

    fn render(&mut self, frame: &mut Frame) {
        let area = frame.area();
        let horizontal_padding = (area.width.saturating_sub(PORFOLIO_WIDTH)) / 2;

        let centered_area = Rect {
            x: horizontal_padding,
            y: area.y,
            width: PORFOLIO_WIDTH,
            height: area.height,
        };

        let content_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(0), // Fill height
                Constraint::Length(INTRO_HEIGHT),
                Constraint::Min(0), // Fill height
                Constraint::Length(ACCOUNT_HEIGHT),
                Constraint::Min(0), // Fill height
                Constraint::Length(BUTTONS_ROW_HEIGHT),
                Constraint::Min(0), // Fill height
            ])
            .split(centered_area);

        let intro_text = Paragraph::new(INTRO_TEXT)
            .style(Style::default().fg(Color::Yellow).bold())
            .alignment(Alignment::Center);
        frame.render_widget(intro_text, content_layout[1]);

        // TODO: Replace with account widget
        let blance = if let Some(balance) = self.balance {
            balance
        } else {
            U256::zero()
        };
        let account_text = Paragraph::new(
            format!("ETH:{}; Balance: {}", self.session.account.to_string(), blance))
            .style(Style::default().fg(Color::Yellow).bold())
            .alignment(Alignment::Center);
        frame.render_widget(account_text, content_layout[3]);

        let buttons_row = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ])
            .split(content_layout[5]);

        self.quit_button.render(frame, buttons_row[0]);
        self.manage_button.render(frame, buttons_row[1]);
    }
}
