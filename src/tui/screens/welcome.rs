
use std::sync::mpsc;
use web3::types::H160;
use ratatui::{
    crossterm::event::Event,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    widgets::Paragraph, Frame
};

use crate::{core::seed_phrase::SeedPhrase, service::session::Session};
use crate::tui::{widgets::{buttons, ascii}, app::{AppCommand, AppScreen}};

const WELCOME_WIDTH: u16 = 80;
const LOGO_HALF_HEIGHT: u16 = 8;
const WARNING_HEIGHT: u16 = 1;
const BUTTONS_ROW_HEIGHT: u16 = 3;

const WARNING_TEXT: &str = "Please don't use this wallet for real crypto!";

enum ProcessActions {
    Login { login_button: buttons::Button, account: H160 },
    Create { import_button: buttons::Button, create_button: buttons::Button }
}

pub struct Screen {
    command_tx: mpsc::Sender<AppCommand>,

    quit_button: buttons::Button,
    process_actions: ProcessActions
}

impl Screen {
    pub fn new(command_tx: mpsc::Sender<AppCommand>) -> Self {
        let quit_button = buttons::Button::new("Quit", Some('q'));
        let process_actions = {
            let accounts = Session::list_accounts().expect("Failed to list accounts");
            match accounts.len() {
                0 => {
                    let import_button = buttons::Button::new("Import Mnemonic", Some('i'));
                    let create_button = buttons::Button::new("Create Account", Some('c'));
                    ProcessActions::Create { create_button, import_button }
                },
                1 => {
                    let account = accounts.first().unwrap().clone();
                    let login_button = buttons::Button::new("Login", Some('l'));
                    ProcessActions::Login { login_button, account }
                },
                _ => panic!("Multiple accounts are not supported yet")
            }
        };

        Self { command_tx, quit_button,process_actions }
    }
}

impl AppScreen for Screen {
    fn handle_event(&mut self, event: Event) -> anyhow::Result<()> {
        if let Some(()) = self.quit_button.handle_event(&event) {
            self.command_tx.send(AppCommand::Quit).unwrap();
            return Ok(());
        }

        match &mut self.process_actions {
            ProcessActions::Login { login_button, account } => {
                if let Some(()) = login_button.handle_event(&event) {
                    let login_screen = Box::new(super::account_login::Screen::new(self.command_tx.clone(), account.clone()));
                    self.command_tx.send(AppCommand::SwitchScreen(login_screen)).unwrap();
                    return Ok(());
                }
            },
            ProcessActions::Create { import_button, create_button } => {
                if let Some(()) = import_button.handle_event(&event) {
                    let import_screen = Box::new(super::account_import_start::Screen::new(self.command_tx.clone()));
                    self.command_tx.send(AppCommand::SwitchScreen(import_screen)).unwrap();
                    return Ok(());
                }
                if let Some(()) = create_button.handle_event(&event) {
                    let seed_phrase = SeedPhrase::generate(bip39::MnemonicType::Words12);
                    let create_screen = Box::new(super::account_create::Screen::new(self.command_tx.clone(), seed_phrase));
                    self.command_tx.send(AppCommand::SwitchScreen(create_screen)).unwrap();
                    return Ok(());
                }
            }
        }
        Ok(())
    }

    fn update(&mut self) {}

    fn render(&mut self, frame: &mut Frame) {
        let area = frame.area();
        let horizontal_padding = (area.width.saturating_sub(WELCOME_WIDTH)) / 2;

        let centered_area = Rect {
            x: horizontal_padding,
            y: area.y,
            width: WELCOME_WIDTH,
            height: area.height,
        };

        let content_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(0), // Fill height
                Constraint::Fill(LOGO_HALF_HEIGHT), // Logo
                Constraint::Length(WARNING_HEIGHT),
                Constraint::Length(BUTTONS_ROW_HEIGHT),
            ])
            .split(centered_area);

        let logo = Paragraph::new(ascii::BIG_LOGO)
            .style(Style::default().fg(Color::Yellow))
            .alignment(Alignment::Center);
        frame.render_widget(logo, content_layout[1]);

        let warning_text = Paragraph::new(WARNING_TEXT)
            .style(Style::default().fg(Color::Red).bold())
            .alignment(Alignment::Center);
        frame.render_widget(warning_text, content_layout[2]);

        match &mut self.process_actions {
            ProcessActions::Login { login_button, .. } => {
                let buttons_row = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(30),
                    Constraint::Percentage(70),
                ])
                .split(content_layout[3]);

                self.quit_button.render(frame, buttons_row[0]);
                login_button.render(frame, buttons_row[1]);
            },
            ProcessActions::Create { import_button, create_button } => {
                let buttons_row = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(30),
                    Constraint::Percentage(35),
                    Constraint::Percentage(35),
                ])
                .split(content_layout[3]);
                
                self.quit_button.render(frame, buttons_row[0]);
                import_button.render(frame, buttons_row[1]);
                create_button.render(frame, buttons_row[2]);
            }
        }
    }
}
