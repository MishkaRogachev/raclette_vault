use std::sync::mpsc;
use ratatui::{
    crossterm::event::Event,
    layout::{Constraint, Direction, Layout, Rect},
    Frame
};

use crate::{core::{chain::Chain, provider::Provider}, service::session::Session};
use crate::tui::{widgets::buttons, app::{AppCommand, AppScreen}};

const PORFOLIO_WIDTH: u16 = 80;
const SWITCH_HEIGHT: u16 = 3;
const BUTTONS_ROW_HEIGHT: u16 = 3;

pub struct Screen {
    command_tx: mpsc::Sender<AppCommand>,
    session: Session,

    mode_switch: buttons::MultiSwitch,
    mode: Option<Box<dyn AppScreen + Send>>,
    quit_button: buttons::Button,
    receive_button: buttons::Button,
    send_button: buttons::Button,
    manage_button: buttons::MenuButton,
    popup: Option<Box<dyn AppScreen + Send>>,
}

impl Screen {
    pub fn new(command_tx: mpsc::Sender<AppCommand>, session: Session) -> Self {
        let infura_token = std::env::var("INFURA_TOKEN")
            .expect("INFURA_TOKEN env var is not set");
        let chain = Chain::EthereumMainnet;
        let provider = Provider::new(&infura_token, chain)
            .expect("Failed to create provider");

        let mode_switch = buttons::MultiSwitch::new(vec![
            buttons::Button::new("Accounts", Some('a')),
            buttons::Button::new("Transactions", Some('t')).disable(),
            buttons::Button::new("Charts", Some('c')).disable(),
            buttons::Button::new("Settings", Some('s')).disable(),
        ]);

        let quit_button = buttons::Button::new("Quit", Some('q'));
        let receive_button = buttons::Button::new("Receive", Some('r'));
        let send_button = buttons::Button::new("Send", Some('s')).disable();

        let networks = buttons::Button::new("Networks", Some('n')).disable();
        let mut access_mnemonic = buttons::Button::new("Access mnemonic", Some('a'));
        if session.get_seed_phrase().is_err() {
            access_mnemonic.disabled = true;
        }
        let delete_account = buttons::Button::new("Delete Account", Some('d')).warning();
        let manage_button = buttons::MenuButton::new(
            "Manage", Some('m'), vec![networks, access_mnemonic, delete_account]
        );

        let mode: Option<Box<dyn AppScreen + Send>> = Some(Box::new(
            super::porfolio_accounts::Screen::new(session.clone(), provider.clone())));

        Self {
            command_tx,
            session,
            mode_switch,
            mode,
            quit_button,
            receive_button,
            send_button,
            manage_button,
            popup: None,
        }
    }
}

#[async_trait::async_trait]
impl AppScreen for Screen {
    fn handle_event(&mut self, event: Event) -> anyhow::Result<bool> {
        if let Some(popup) = &mut self.popup {
            if let Ok(ok) = popup.handle_event(event) {
                if ok {
                    self.popup = None;
                    return Ok(true);
                }
            }
            return Ok(false);
        }

        if let Some(index) = self.manage_button.handle_event(&event) {
            match index {
                0 => {
                    // self.command_tx.send(AppCommand::SwitchScreen(Box::new(
                    //     super::networks::Screen::new(self.command_tx.clone(), self.session.clone())
                    // ))).unwrap();
                    return Ok(true);
                },
                1 => {
                    self.command_tx.send(AppCommand::SwitchScreen(Box::new(
                        super::mnemonic_access::Screen::new(self.command_tx.clone(), self.session.clone())
                    ))).unwrap();
                    return Ok(true);
                },
                2 => {
                    self.command_tx.send(AppCommand::SwitchScreen(Box::new(
                        super::account_delete::Screen::new(
                            self.command_tx.clone(), self.session.clone())
                    ))).unwrap();
                    return Ok(true);
                },
                _ => {}
            }
            return Ok(false);
        }

        if let Some(index) = self.mode_switch.handle_event(&event) {
            match index {
                _ => {} // TODO: implement modes
            }
            return Ok(false);
        }

        if let Some(mode) = &mut self.mode {
            if let Ok(ok) = mode.handle_event(event.clone()) {
                if ok {
                    return Ok(true);
                }
            }
        }

        if let Some(()) = self.receive_button.handle_event(&event) {
            self.popup = Some(Box::new(super::super::popups::receive::Popup::new(self.session.account)));
            return Ok(true);
        }

        if let Some(()) = self.send_button.handle_event(&event) {
            // TODO
            return Ok(true);
        }

        if let Some(()) = self.quit_button.handle_event(&event) {
            self.command_tx.send(AppCommand::Quit).unwrap();
            return Ok(true);
        }

        return Ok(false);
    }

    async fn update(&mut self) {
        if let Some(mode) = &mut self.mode {
            mode.update().await;
        }
    }

    fn render(&mut self, frame: &mut Frame, area: Rect) {
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
                Constraint::Length(SWITCH_HEIGHT),
                Constraint::Fill(0), // Fill height for mode
                Constraint::Length(BUTTONS_ROW_HEIGHT),
            ])
            .split(centered_area);

        self.mode_switch.render(frame, content_layout[0]);

        if let Some(mode) = &mut self.mode {
            mode.render(frame, content_layout[1]);
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
            popup.render(frame, area);
        }
    }
}
