
use std::sync::Arc;
use tokio::sync::Mutex;
use ratatui::{
    crossterm::event::Event,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame
};

use crate::{core::chain, service::crypto::Crypto};
use crate::tui::{app::AppScreen, widgets::{buttons, options}};

const TITLE: &str = "Active Networks";
const TESTNET_WARNING: &str = "Testnet mode";

const WARNING_HEIGHT: u16 = 1;

pub struct Popup {
    crypto: Arc<Mutex<Crypto>>,
    is_testnet: bool,
    update_required: bool,
    has_changes: bool,
    mainnet_options: options::CheckOptions<chain::Chain>,
    testnet_options: options::CheckOptions<chain::Chain>,
    back_button: buttons::Button,
    restore_button: buttons::Button,
    save_button: buttons::Button,
}

impl Popup {
    pub fn new(crypto: Arc<Mutex<Crypto>>) -> Self {
        let mainnet_options = chain::MAINNET_CHAINS.iter().map(|chain| {
            let name = chain.get_display_name();
            let hotkey = name.chars().next();
            (chain.clone(), options::CheckBox::new(name, false, hotkey))
        }).collect();
        let mainnet_options = options::CheckOptions::new(mainnet_options);

        let testnet_options = chain::TESTNET_CHAINS.iter().map(|chain| {
            let name = chain.get_display_name();
            let hotkey = name.chars().next();
            (chain.clone(), options::CheckBox::new(name, false, hotkey).warning())
        }).collect();
        let testnet_options = options::CheckOptions::new(testnet_options);

        let back_button = buttons::Button::new("Back", Some('b'));
        let restore_button = buttons::Button::new("Restore", Some('r')).disable();
        let save_button = buttons::Button::new("Save", Some('s')).disable();

        Self {
            crypto,
            is_testnet: false,
            update_required: true,
            has_changes: false,
            mainnet_options,
            testnet_options,
            back_button,
            restore_button,
            save_button
        }
    }

    async fn update_options(&mut self) {
        let crypto = self.crypto.lock().await;
        let active_networks = crypto.get_active_networks();
        self.is_testnet = crypto.in_testnet();

        self.mainnet_options.toggle_by_keys(&active_networks);
        self.testnet_options.toggle_by_keys(&active_networks);
    }

    async fn save_options(&mut self) {
        let mut crypto = self.crypto.lock().await;
        let mut all_networks = self.mainnet_options.get_checked_keys();
        all_networks.extend(self.testnet_options.get_checked_keys());
        crypto.save_active_networks(all_networks)
            .await.expect("Failed to save active networks");
    }
}

#[async_trait::async_trait]
impl AppScreen for Popup {
    async fn handle_event(&mut self, event: Event) -> anyhow::Result<bool> {
        if let Some(_) = self.mainnet_options.handle_event(&event) {
            self.has_changes = true;
            return Ok(false);
        }

        if let Some(_) = self.testnet_options.handle_event(&event) {
            self.has_changes = true;
            return Ok(false);
        }

        if let Some(()) = self.back_button.handle_event(&event) {
            return Ok(true);
        }
        if let Some(()) = self.restore_button.handle_event(&event) {
            self.update_required = true;
            return Ok(false);
        }
        if let Some(()) = self.save_button.handle_event(&event) {
            self.save_options().await;
            self.has_changes = false;
            self.update_required = true;
            return Ok(true);
        }
        Ok(false)
    }

    async fn update(&mut self) {
        if self.update_required {
            self.update_options().await;
            self.update_required = false;
            self.has_changes = false;
        }

        self.restore_button.disabled = !self.has_changes;
        self.save_button.disabled = !self.has_changes;
    }

    fn render(&mut self, frame: &mut Frame, area: Rect) {
        frame.render_widget(Clear, area);

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Yellow))
            .title(TITLE);
        let inner_area = block.inner(area);
        frame.render_widget(block, area);

        let content_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1), // Margin
                Constraint::Length(WARNING_HEIGHT),
                Constraint::Fill(0), // Fill height for network options
                Constraint::Length(buttons::BUTTONS_HEIGHT),
            ])
            .split(inner_area);

        if self.is_testnet {
            let warning_label = Paragraph::new(TESTNET_WARNING)
                .style(Style::default().fg(Color::Red).add_modifier(ratatui::style::Modifier::BOLD))
                .alignment(Alignment::Center);
            frame.render_widget(warning_label, content_layout[1]);
        }

        let options_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(10),
                Constraint::Percentage(40),
                Constraint::Percentage(40),
                Constraint::Percentage(10),
            ])
            .split(content_layout[2]);
        self.mainnet_options.render(frame, options_layout[1]);
        self.testnet_options.render(frame, options_layout[2]);

        let buttons_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(30),
                Constraint::Percentage(35),
                Constraint::Percentage(35),
            ])
            .split(content_layout[3]);
        frame.render_widget(Clear, content_layout[3]);

        self.back_button.render(frame, buttons_layout[0]);
        self.restore_button.render(frame, buttons_layout[1]);
        self.save_button.render(frame, buttons_layout[2]);
    }
}
