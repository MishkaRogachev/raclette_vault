use std::sync::{atomic::AtomicBool, Arc};
use flexi_logger::{Logger, FileSpec, WriteMode, DeferredNow, Record};

mod utils;
mod core;
mod persistence;
mod service;
mod tui;

fn log_format(w: &mut dyn std::io::Write, now: &mut DeferredNow, record: &Record) -> std::io::Result<()> {
    write!(
        w,
        "[{}] {}({}) {}",
        now.format("%d.%m.%Y-%H:%M:%S"),
        record.level(),
        record.module_path().unwrap_or("<unknown module>"),
        &record.args()
    )
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    if std::env::var("INFURA_TOKEN").is_err() {
        panic!("INFURA_TOKEN env variable is not set!");
    }

    let mut log_dir = utils::app_data_path()?;
    log_dir.push("logs");
    std::fs::create_dir_all(&log_dir).unwrap();

    Logger::try_with_str("info")
        .unwrap()
        .log_to_file(FileSpec::default().directory(log_dir))
        .format(log_format)
        .write_mode(WriteMode::BufferAndFlush)
        .start()
        .unwrap();

    let shutdown_handle = Arc::new(AtomicBool::new(false));

    let events = tui::event::EventHandler::new(shutdown_handle.clone());
    let app = tui::app::App::new(shutdown_handle.clone(), events.subscribe_events())?;
    let tui = tui::Tui::new(shutdown_handle, app)?;

    tokio::select! {
        _ = tui.run() => {},
        _ = events.handle_events() => {},
    }

    Ok(())
}
