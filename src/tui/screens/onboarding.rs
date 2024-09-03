
use ratatui::{
    crossterm::event::Event,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::Paragraph, Frame
};

use super::super::common;

pub struct OnboardingScreen {
    quit_button: common::Button,
    next_button: common::Button,
}

impl OnboardingScreen {
    pub fn new(shutdown_handle: std::sync::Arc<std::sync::atomic::AtomicBool>) -> Self {
        let quit_button = common::Button::new("Quit", Some('q'))
            .action(move || { shutdown_handle.store(true, std::sync::atomic::Ordering::Relaxed); });

        let next_button = common::Button::new("Next", Some('n'));

        Self { quit_button, next_button }
    }
}

impl common::Widget for OnboardingScreen {
    fn handle_event(&mut self, event: Event) -> Option<Event> {
        let event = self.quit_button.handle_event(event);
        if let Some(event) = event {
            return self.next_button.handle_event(event);
        }
        None
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) {
        let content = Paragraph::new("Onboarding")
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Center);

        frame.render_widget(content, area);

        let buttons_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Ratio(1, 2),
                Constraint::Ratio(1, 2),
            ])
            .split(area);

        self.quit_button.draw(frame, buttons_layout[0]);
        self.next_button.draw(frame, buttons_layout[1]);
    }
}
