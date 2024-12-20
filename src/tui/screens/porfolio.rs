use std::{collections::HashMap, sync::{mpsc, Arc}};
use tokio::sync::Mutex;
use ratatui::{
    crossterm::event::Event,
    layout::{Constraint, Direction, Layout, Rect},
    Frame
};

use crate::service::{crypto::Crypto, session::Session};
use crate::tui::{widgets::controls, app::{AppCommand, AppScreen}};

const POPUP_WIDTH: u16 = 60;
const POPUP_HEIGHT: u16 = 30;

#[derive(Clone, PartialEq, Eq, Hash)]
enum ManageOption {
    Networks,
    AccessMnemonic,
    DeleteAccount,
}

pub trait PorfolioPage: AppScreen {
    fn on_networks_change(&mut self);
}

pub struct Screen {
    command_tx: mpsc::Sender<AppCommand>,
    session: Session,
    crypto: Arc<Mutex<Crypto>>,

    page_switch: controls::MultiSwitch,
    page: Option<Box<dyn PorfolioPage + Send>>,
    quit_button: controls::Button,
    receive_button: controls::Button,
    send_button: controls::Button,
    manage_button: controls::MenuButton<ManageOption>,
    popup: Option<Box<dyn AppScreen + Send>>,
}

impl Screen {
    pub fn new(command_tx: mpsc::Sender<AppCommand>, session: Session) -> Self {
        let infura_token = std::env::var("INFURA_TOKEN")
            .expect("INFURA_TOKEN env var is not set");
        let endpoint_url = format!("infura.io/v3/{}", infura_token);

        let mut crypto: Crypto = Crypto::new(session.db.clone(), &endpoint_url);
        crypto.load_active_networks().expect("Failed to load active networks");
        let crypto = Arc::new(Mutex::new(crypto));

        let page_switch = controls::MultiSwitch::new(vec![
            controls::Button::new("Accounts", Some('a')),
            controls::Button::new("Transactions", Some('t')),
            controls::Button::new("Charts", Some('c')).disable(),
            controls::Button::new("Settings", Some('s')).disable(),
        ]);

        let quit_button = controls::Button::new("Quit", Some('q')).escape();
        let receive_button = controls::Button::new("Receive", Some('r'));
        let send_button = controls::Button::new("Send", Some('s'));

        let mut manage_options = HashMap::new();
        if session.db.get_seed_phrase().is_ok() {
            manage_options.insert(ManageOption::AccessMnemonic, "Access mnemonic".to_string());
        }
        manage_options.insert(ManageOption::Networks, "Networks".to_string());
        manage_options.insert(ManageOption::DeleteAccount, "Delete Account".to_string());
        let manage_button = controls::MenuButton::new(
            "Manage", Some('m'), manage_options).keep_above();

        let page: Option<Box<dyn PorfolioPage + Send>> = Some(Box::new(
            super::porfolio_accounts::Page::new(session.clone(), crypto.clone())));

        Self {
            command_tx,
            session,
            crypto,
            page_switch,
            page,
            quit_button,
            receive_button,
            send_button,
            manage_button,
            popup: None,
        }
    }

    pub fn on_networks_change(&mut self) {
        if let Some(page) = &mut self.page {
            page.on_networks_change();
        }
    }
}

#[async_trait::async_trait]
impl AppScreen for Screen {
    async fn handle_event(&mut self, event: Event) -> anyhow::Result<bool> {
        if let Some(popup) = &mut self.popup {
            if let Ok(ok) = popup.handle_event(event).await {
                if ok {
                    self.popup = None;
                    // TODO: check if networks have changed indeed
                    self.on_networks_change();
                    return Ok(true);
                }
            }
            return Ok(false);
        }

        if let Some(manage_event) = self.manage_button.handle_event(&event) {
            if let controls::MenuEvent::Selected(manage_option) = manage_event {
                match manage_option {
                    ManageOption::Networks => {
                        self.popup = Some(Box::new(super::super::popups::networks::Popup::new(self.crypto.clone())));
                        return Ok(true);
                    },
                    ManageOption::AccessMnemonic => {
                        self.command_tx.send(AppCommand::SwitchScreen(Box::new(
                            super::mnemonic_access::Screen::new(self.command_tx.clone(), self.session.clone())
                        ))).unwrap();
                        return Ok(true);
                    },
                    ManageOption::DeleteAccount => {
                        self.command_tx.send(AppCommand::SwitchScreen(Box::new(
                            super::account_delete::Screen::new(
                                self.command_tx.clone(), self.session.clone())
                        ))).unwrap();
                        return Ok(true);
                    }
                }
            }
            return Ok(false);
        }

        if let Some(index) = self.page_switch.handle_event(&event) {
            // TODO: switch to enum
            match index {
                0 => {
                    self.page = Some(Box::new(
                        super::porfolio_accounts::Page::new(self.session.clone(), self.crypto.clone())
                    ));
                },
                1 => {
                    self.page = Some(Box::new(
                        super::porfolio_transactions::Page::new(self.session.clone())
                    ));
                },
                _ => {} // TODO: other pages
            }
            return Ok(false);
        }

        if let Some(page) = &mut self.page {
            if let Ok(ok) = page.handle_event(event.clone()).await {
                if ok {
                    return Ok(true);
                }
            }
        }

        if let Some(()) = self.receive_button.handle_event(&event) {
            self.popup = Some(Box::new(super::super::popups::transaction_receive::Popup::new(self.session.account)));
            return Ok(true);
        }

        if let Some(()) = self.send_button.handle_event(&event) {
            let popup = super::super::popups::transaction_send::Popup::new(self.session.clone(), self.crypto.clone()).await;
            self.popup = Some(Box::new(popup));
            return Ok(true);
        }

        if let Some(()) = self.quit_button.handle_event(&event) {
            self.command_tx.send(AppCommand::Quit).unwrap();
            return Ok(true);
        }

        return Ok(false);
    }

    async fn update(&mut self) {
        if let Some(popup) = &mut self.popup {
            popup.update().await;
        }

        if let Some(page) = &mut self.page {
            page.update().await;
        }
    }

    fn render(&mut self, frame: &mut Frame, area: Rect) {
        let content_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(controls::SWITCH_HEIGHT),
                Constraint::Fill(0), // Fill height for page
                Constraint::Length(controls::BUTTON_HEIGHT),
            ])
            .split(area);

        self.page_switch.render(frame, content_layout[0]);

        if let Some(page) = &mut self.page {
            page.render(frame, content_layout[1]);
        }

        let buttons_row = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
            ])
            .split(content_layout[2]);

        self.quit_button.render(frame, buttons_row[0]);
        self.receive_button.render(frame, buttons_row[1]);
        self.send_button.render(frame, buttons_row[2]);
        self.manage_button.render(frame, buttons_row[3]);

        if let Some(popup) = &mut self.popup {
            let popup_h_padding = area.width.saturating_sub(POPUP_WIDTH) / 2;
            let popup_height = POPUP_HEIGHT.min(area.height);
            let popup_v_padding = area.height.saturating_sub(popup_height) / 2;
            let popup_area = Rect {
                x: area.x + popup_h_padding,
                y: area.y + popup_v_padding,
                width: POPUP_WIDTH,
                height: popup_height,
            };
            popup.render(frame, popup_area);
        }
    }
}
