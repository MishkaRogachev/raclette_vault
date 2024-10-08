
use std::sync::mpsc;
use ratatui::{
    crossterm::event::Event,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    widgets::Paragraph, Frame
};

use crate::{core::seed_phrase::{WordCount, SeedPhrase}, service::session::Session};
use crate::tui::{widgets::{controls, ascii}, app::{AppCommand, AppScreen}};

const LOGO_HEIGHT: u16 = 20;
const WARNING_HEIGHT: u16 = 1;

const WARNING_TEXT: &str = "Please don't use this wallet for real crypto!";

enum ProcessActions {
    Login { login_button: controls::Button, account: web3::types::Address },
    Create { import_button: controls::Button, create_button: controls::Button }
}

pub struct Screen {
    command_tx: mpsc::Sender<AppCommand>,

    quit_button: controls::Button,
    process_actions: ProcessActions
}

impl Screen {
    pub fn new(command_tx: mpsc::Sender<AppCommand>) -> Self {
        let quit_button = controls::Button::new("Quit", Some('q')).escape();
        let process_actions = {
            let accounts = Session::list_accounts().expect("Failed to list accounts");
            match accounts.len() {
                0 => {
                    let import_button = controls::Button::new("Import Mnemonic", Some('i'));
                    let create_button = controls::Button::new("Create Account", Some('c')).default();
                    ProcessActions::Create { create_button, import_button }
                },
                1 => {
                    let account = accounts.first().unwrap().clone();
                    let login_button = controls::Button::new("Login", Some('l')).default();
                    ProcessActions::Login { login_button, account }
                },
                _ => panic!("Multiple accounts are not supported yet")
            }
        };

        Self { command_tx, quit_button,process_actions }
    }
}

#[async_trait::async_trait]
impl AppScreen for Screen {
    async fn handle_event(&mut self, event: Event) -> anyhow::Result<bool> {
        if let Some(()) = self.quit_button.handle_event(&event) {
            self.command_tx.send(AppCommand::Quit).unwrap();
            return Ok(true);
        }

        match &mut self.process_actions {
            ProcessActions::Login { login_button, account } => {
                if let Some(()) = login_button.handle_event(&event) {
                    let login_screen = Box::new(super::account_login::Screen::new(self.command_tx.clone(), account.clone()));
                    self.command_tx.send(AppCommand::SwitchScreen(login_screen)).unwrap();
                    return Ok(true);
                }
            },
            ProcessActions::Create { import_button, create_button } => {
                if let Some(()) = import_button.handle_event(&event) {
                    let import_screen = Box::new(super::account_import_start::Screen::new(self.command_tx.clone()));
                    self.command_tx.send(AppCommand::SwitchScreen(import_screen)).unwrap();
                    return Ok(true);
                }
                if let Some(()) = create_button.handle_event(&event) {
                    let seed_phrase = SeedPhrase::generate(WordCount::Words12)?;
                    let create_screen = Box::new(super::account_create::Screen::new(self.command_tx.clone(), seed_phrase));
                    self.command_tx.send(AppCommand::SwitchScreen(create_screen)).unwrap();
                    return Ok(true);
                }
            }
        }
        Ok(false)
    }

    async fn update(&mut self) {}

    fn render(&mut self, frame: &mut Frame, area: Rect) {
        let content_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(0), // Fill height
                Constraint::Max(LOGO_HEIGHT),
                Constraint::Length(WARNING_HEIGHT),
                Constraint::Length(controls::BUTTON_HEIGHT),
                Constraint::Min(0), // Fill height
            ])
            .split(area);

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
                    Constraint::Fill(0),
                    Constraint::Length(30),
                    Constraint::Length(30),
                    Constraint::Fill(0),
                ])
                .split(content_layout[3]);

                self.quit_button.render(frame, buttons_row[1]);
                login_button.render(frame, buttons_row[2]);
            },
            ProcessActions::Create { import_button, create_button } => {
                let buttons_row = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Fill(0),
                    Constraint::Length(20),
                    Constraint::Length(20),
                    Constraint::Length(20),
                    Constraint::Fill(0),
                ])
                .split(content_layout[3]);
                
                self.quit_button.render(frame, buttons_row[1]);
                import_button.render(frame, buttons_row[2]);
                create_button.render(frame, buttons_row[3]);
            }
        }
    }
}
