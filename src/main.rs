use std::sync::{atomic::AtomicBool, Arc};

mod core;
mod tui;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let shutdown_handle = Arc::new(AtomicBool::new(false));

    let events = tui::event::EventHandler::new(shutdown_handle);
    let tui = tui::Tui::new(events.clone())?;
    let app = tui::app::App::new()?;

    tokio::select! {
        _ = tui.run(app)? => {},
        _ = events.handle_events() => {},
        _ = tokio::signal::ctrl_c() => {},
    }

    Ok(())
}
