use std::sync::{atomic::{AtomicBool, Ordering}, Arc};

use ratatui::{
    crossterm::{event as crossterm_event, terminal, ExecutableCommand},
    prelude::CrosstermBackend, Terminal
};

use super::{event::EventHandler, common::Widget};

const MIN_TERMINAL_WIDTH: u16 = 60;
const MIN_TERMINAL_HEIGHT: u16 = 12;

pub struct Tui {
    handler: EventHandler,
    shutdown_handle: Arc<AtomicBool>,
}

impl Tui {
    pub fn new(handler: EventHandler) -> anyhow::Result<Self> {
        setup_terminal()?;

        let panic_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |panic| {
            restore_terminal().expect("failed to restore the terminal");
            panic_hook(panic);
        }));

        let shutdown_handle = Arc::new(AtomicBool::new(false));

        Ok(Self { handler, shutdown_handle })
    }

    pub fn run(&self) -> anyhow::Result<tokio::task::JoinHandle<()>> {
        let shutdown_handle = self.shutdown_handle.clone();
        let mut events = self.handler.subscribe_events();
        let mut terminal = Terminal::new(CrosstermBackend::new(std::io::stdout()))?;

        Ok(tokio::spawn(async move {
            let mut screen = super::screens::welcome::WelcomeScreen::new(shutdown_handle.clone());
            while !shutdown_handle.load(Ordering::Relaxed) {
                if let Ok(event) = events.try_recv() {
                    screen.handle_event(event);
                }

                terminal.draw(|frame| {
                    let area = frame.area();

                    if area.width < MIN_TERMINAL_WIDTH || area.height < MIN_TERMINAL_HEIGHT {
                        let warning = ratatui::widgets::Paragraph::new("Terminal window is too small")
                            .alignment(ratatui::layout::Alignment::Center);
                        frame.render_widget(warning, area);
                    } else {
                        screen.draw(frame, area);
                    }
                }).unwrap();
            }
        }))
    }

    pub fn stop(&mut self) -> anyhow::Result<()> {
        self.shutdown_handle.store(true, Ordering::Relaxed);
        Ok(())
    }
}

impl Drop for Tui {
    fn drop(&mut self) {
        restore_terminal().expect("failed to reset the terminal");
        self.stop().expect("failed to stop the tui");
    }
}

fn setup_terminal() -> anyhow::Result<()> {
    terminal::enable_raw_mode()?;
    std::io::stdout().execute(terminal::EnterAlternateScreen)?;
    std::io::stdout().execute(crossterm_event::EnableMouseCapture)?;
    Ok(())
}

fn restore_terminal() -> anyhow::Result<()> {
    terminal::disable_raw_mode()?;
    std::io::stdout().execute(crossterm_event::DisableMouseCapture)?;
    std::io::stdout().execute(terminal::LeaveAlternateScreen)?;
    Ok(())
}
