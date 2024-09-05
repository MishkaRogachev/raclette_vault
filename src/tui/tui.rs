use ratatui::{
    crossterm::{event::{self as crossterm_event}, terminal, ExecutableCommand},
    prelude::CrosstermBackend, Terminal
};

use super::{app::App, widgets::common::ControlTrait, event::EventHandler};

const MIN_TERMINAL_WIDTH: u16 = 60;
const MIN_TERMINAL_HEIGHT: u16 = 14;

pub struct Tui {
    handler: EventHandler,
}

impl Tui {
    pub fn new(handler: EventHandler) -> anyhow::Result<Self> {
        setup_terminal()?;

        let panic_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |panic| {
            restore_terminal().expect("failed to restore the terminal");
            panic_hook(panic);
        }));

        Ok(Self { handler })
    }

    pub fn run(&self, mut app: App) -> anyhow::Result<tokio::task::JoinHandle<()>> {
        let mut events = self.handler.subscribe_events();
        let mut terminal = Terminal::new(CrosstermBackend::new(std::io::stdout()))?;

        Ok(tokio::spawn(async move {
            while app.is_running() {
                if let Ok(event) = events.try_recv() {
                    app.handle_event(event);
                }

                terminal.draw(|frame| {
                    let area = frame.area();

                    if area.width < MIN_TERMINAL_WIDTH || area.height < MIN_TERMINAL_HEIGHT {
                        let warning = ratatui::widgets::Paragraph::new("Terminal window is too small")
                            .alignment(ratatui::layout::Alignment::Center);
                        frame.render_widget(warning, area);
                    } else {
                        app.draw(frame, area);
                    }
                }).unwrap();
                app.check_app_commands().expect("failed to check app commands");
            }
        }))
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
    std::io::stdout().execute(terminal::LeaveAlternateScreen)?;
    std::io::stdout().execute(crossterm_event::PopKeyboardEnhancementFlags)?;
    Ok(())
}
