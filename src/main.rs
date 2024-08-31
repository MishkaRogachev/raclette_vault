use std::{io, io::stdout};
use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        event::{self, Event, KeyCode},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    Terminal,
};

mod core;
mod ui;

const MIN_TERMINAL_WIDTH: u16 = 60;
const MIN_TERMINAL_HEIGHT: u16 = 12;

fn main() -> io::Result<()> {
    // Set up terminal
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;

    // Main loop
    let mut should_quit = false;
    while !should_quit {
        terminal.draw(|f| {
            let area = f.area();
            if area.width < MIN_TERMINAL_WIDTH || area.height < MIN_TERMINAL_HEIGHT {
                let warning = ratatui::widgets::Paragraph::new("Terminal window is too small")
                    .alignment(ratatui::layout::Alignment::Center);
                f.render_widget(warning, area);
            } else {
                ui::welcome::welcome_new_user(f);
            }
        })?;
        should_quit = handle_events()?;
    }

    // Restore terminal
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

fn handle_events() -> io::Result<bool> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.code == KeyCode::Char('q') {
                return Ok(true);
            }
        }
    }
    Ok(false)
}
