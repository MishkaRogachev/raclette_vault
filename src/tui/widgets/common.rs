
use ratatui::{ crossterm::event::{Event, MouseEvent}, layout::{Position, Rect}, Frame };

pub trait Widget {
    fn handle_event(&mut self, event: Event) -> Option<Event>;
    fn draw(&mut self, frame: &mut Frame, area: ratatui::layout::Rect);
}

pub struct Control {
    pub is_hovered: bool,
    pub is_down: bool,
    pub area: Option<Rect>,
}

impl Control {
    pub fn new() -> Self {
        Control {
            is_hovered: false,
            is_down: false,
            area: None,
        }
    }

    pub fn contains(&self, x: u16, y: u16) -> bool {
        if let Some(area) = self.area {
            return area.contains(Position { x, y });
        }
        false
    }

    pub fn handle_mouse_hover(&mut self, mouse_event: MouseEvent) {
        self.is_hovered = self.contains(mouse_event.column, mouse_event.row);
    }
}

