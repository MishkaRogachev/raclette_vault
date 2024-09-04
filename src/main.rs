mod core;
mod tui;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let handler = tui::event::EventHandler::new();
    let tui = tui::Tui::new(handler.clone())?;
    let app = tui::app::App::new()?;

    tokio::select! {
        _ = tui.run(app)? => {},
        _ = handler.handle_events() => {},
        _ = tokio::signal::ctrl_c() => {},
    }

    Ok(())
}
