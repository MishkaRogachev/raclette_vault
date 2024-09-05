use std::sync::mpsc;
use ratatui::{crossterm::event::{Event, KeyCode, KeyModifiers}, layout::Rect, Frame};

#[derive(Clone, Debug)]
pub enum AppScreenType {
    Welcome,
    Generate,
//    Secure,
//    Home,
//    Accounts,
}

#[derive(Clone, Debug)]
pub enum AppCommand {
    SwitchScreen(AppScreenType),
    Quit,
}

pub struct App {
    current_screen: Box<dyn super::widgets::common::Widget + Send>,
    command_tx: mpsc::Sender<AppCommand>,
    command_rx: mpsc::Receiver<AppCommand>,
    running: bool,
}

impl App {
    pub fn new() -> anyhow::Result<Self> {
        let (command_tx, command_rx) = mpsc::channel();

        let current_screen = Box::new(
            super::screens::welcome::WelcomeScreen::new(command_tx.clone())?
        );

        Ok(Self { current_screen, command_tx, command_rx, running: true })
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn check_app_commands(&mut self) -> anyhow::Result<()> {
        if let Ok(command) = self.command_rx.try_recv() {
            match command {
                AppCommand::SwitchScreen(screen_type) => {
                    match screen_type {
                        AppScreenType::Welcome => {
                            self.current_screen = Box::new(
                                super::screens::welcome::WelcomeScreen::new(self.command_tx.clone())?
                            );
                        },
                        AppScreenType::Generate => {
                            self.current_screen = Box::new(
                                super::screens::generate::GeneratePhraseScreen::new(self.command_tx.clone())?
                            );
                        },
                    }
                },
                AppCommand::Quit => {
                    self.running = false;
                },
            }
        }
        Ok(())
    }
}

impl super::widgets::common::Widget for App {
    fn handle_event(&mut self, event: Event) -> Option<Event> {
        if let Event::Key(key_event) = event {
            match key_event.code {
                // Exit application on `Ctrl-C`
                KeyCode::Char('c') | KeyCode::Char('C') => {
                    if key_event.modifiers == KeyModifiers::CONTROL {
                        self.running = false;
                    }
                },
                _ => {}
            }
        }

        self.current_screen.handle_event(event)
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) {
        self.current_screen.draw(frame, area);
    }
}