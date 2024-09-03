use std::sync::{atomic::{AtomicBool, Ordering}, Arc};

use ratatui::{crossterm::event::Event, layout::Rect, Frame};

pub struct App {
    shutdown_handle: Arc<AtomicBool>,
    onboard_handle: Arc<AtomicBool>,
    current_screen: Box<dyn super::common::Widget + Send>,
}

impl App {
    pub fn new() -> Self {
        let shutdown_handle = Arc::new(AtomicBool::new(false));
        let onboard_handle = Arc::new(AtomicBool::new(false));
        let current_screen = Box::new(
            super::screens::welcome::WelcomeScreen::new(shutdown_handle.clone(), onboard_handle.clone())
        );

        Self {
            shutdown_handle,
            onboard_handle,
            current_screen
        }
    }

    pub fn is_running(&self) -> bool {
        !self.shutdown_handle.load(Ordering::Relaxed)
    }

    pub fn check_switch_screen(&mut self) {
        if self.onboard_handle.load(Ordering::Relaxed) {
            self.current_screen = Box::new(
                super::screens::onboarding::OnboardingScreen::new(self.shutdown_handle.clone())
            );
            self.onboard_handle.store(false, Ordering::Relaxed);
        }
    }
}

impl super::common::Widget for App {
    fn handle_event(&mut self, event: Event) -> Option<Event> {
        self.current_screen.handle_event(event)
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) {
        self.current_screen.draw(frame, area);
    }
}