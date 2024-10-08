use std::sync::{atomic::{AtomicBool, Ordering}, Arc};

use ratatui::{
    crossterm::{event::{self as crossterm_event}, terminal, ExecutableCommand},
    prelude::CrosstermBackend, Terminal
};

use super::app::{App, AppScreen, MIN_APP_WIDTH};

const MIN_TERMINAL_HEIGHT: u16 = 13;

const TERMINAL_TOO_SMALL: &str = "Terminal window is too small";

pub struct Tui {
    shutdown_handle: Arc<AtomicBool>,
    app: App
}

impl Tui {
    pub fn new(shutdown_handle: Arc<AtomicBool>, app: App) -> anyhow::Result<Self> {
        setup_terminal()?;

        let panic_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |panic| {
            restore_terminal().expect("failed to restore the terminal");
            panic_hook(panic);
        }));

        Ok(Self { shutdown_handle, app })
    }

    pub fn run(mut self) -> tokio::task::JoinHandle<()> {
        let mut terminal = Terminal::new(CrosstermBackend::new(std::io::stdout()))
            .expect("Failed to create terminal");

        tokio::spawn(async move {
            while !self.shutdown_handle.load(Ordering::Relaxed) {
                self.app.process_events().await;
                self.app.update().await;
                terminal.draw(|frame| {
                    let area = frame.area();
                    if area.width < MIN_APP_WIDTH || area.height < MIN_TERMINAL_HEIGHT {
                        let layout = ratatui::layout::Layout::default()
                            .direction(ratatui::layout::Direction::Vertical)
                            .constraints([
                                ratatui::layout::Constraint::Percentage(50),
                                ratatui::layout::Constraint::Length(1),
                                ratatui::layout::Constraint::Percentage(50),
                            ])
                            .split(area);

                        let warning = ratatui::widgets::Paragraph::new(
                            TERMINAL_TOO_SMALL)
                            .alignment(ratatui::layout::Alignment::Center);

                        frame.render_widget(warning, layout[1]);

                    } else {
                        self.app.render(frame, frame.area());
                    }
                }).expect("failed to draw the frame");
            }
        })
    }

    pub fn stop(&mut self) -> anyhow::Result<()> {
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
    std::io::stdout().execute(crossterm_event::PopKeyboardEnhancementFlags)?;
    std::io::stdout().execute(crossterm_event::PushKeyboardEnhancementFlags(
        crossterm_event::KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES
        | crossterm_event::KeyboardEnhancementFlags::REPORT_EVENT_TYPES
    ))?;
    Ok(())
}

fn restore_terminal() -> anyhow::Result<()> {
    terminal::disable_raw_mode()?;
    std::io::stdout().execute(crossterm_event::DisableMouseCapture)?;
    std::io::stdout().execute(crossterm_event::PopKeyboardEnhancementFlags)?;
    std::io::stdout().execute(terminal::LeaveAlternateScreen)?;
    Ok(())
}
