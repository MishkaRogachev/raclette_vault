use std::sync::{atomic::AtomicBool, Arc};

mod core;
mod tui;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let shutdown_handle = Arc::new(AtomicBool::new(false));

    let events = tui::event::EventHandler::new(shutdown_handle.clone());
    let app = tui::app::App::new(shutdown_handle.clone(), events.subscribe_events())?;
    let tui = tui::Tui::new(shutdown_handle, app)?;

    tokio::select! {
        _ = tui.run()? => {},
        _ = events.handle_events() => {},
    }

    Ok(())
}
