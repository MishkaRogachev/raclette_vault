mod core;
mod tui;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let handler = tui::event::EventHandler::new();
    let tui = tui::Tui::new(handler.clone())?;

    tokio::select! {
        _ = tui.run()? => {},
        _ = handler.handle_events() => {},
        _ = tokio::signal::ctrl_c() => {},
    }

    Ok(())
}
