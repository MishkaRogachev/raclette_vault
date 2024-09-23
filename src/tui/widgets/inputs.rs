use aes_gcm::Key;
use zeroize::Zeroizing;
use ratatui::{
    crossterm::event::{Event, KeyCode},
    layout::{Position, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame
};

use super::focus::{Focusable, FocusableEvent};

pub struct Input {
    pub value: Zeroizing<String>,
    pub placeholder: String,
    pub disabled: bool,
    pub color: Color,
    pub focused: bool,
    pub masked: bool,
    area: Rect,
}

impl Input {
    pub fn new(placeholder: &str) -> Self {
        Self {
            value: Zeroizing::new(String::new()),
            placeholder: placeholder.to_string(),
            disabled: false,
            color: Color::Yellow,
            focused: false,
            masked: false,
            area: Rect::default()
        }
    }

    pub fn masked(mut self) -> Self {
        self.masked = true;
        self
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        self.area = area; // Store the button's area for mouse handling

        let style = if self.focused {
            Style::default().fg(Color::Black).bg(self.color)
        } else {
            Style::default().fg(self.color)
        };

        let block = Block::default().borders(Borders::ALL).border_style(style);
        let display_value = if self.value.is_empty() {
            Span::styled(&self.placeholder, Style::default().fg(Color::DarkGray))
        } else {
            if self.masked {
                Span::styled("*".repeat(self.value.len()), style)
            } else {
                let value: &str = &self.value;
                Span::styled(value, style)
            }
        };

        let paragraph = Paragraph::new(Line::from(display_value)).block(block);
        frame.render_widget(paragraph, area);
    }
}

impl Focusable for Input {
    fn is_focused(&self) -> bool {
        self.focused
    }

    fn set_focused(&mut self, focused: bool) {
        self.focused = focused;
    }

    fn contains(&self, column: u16, row: u16) -> bool {
        self.area.contains(Position { x: column, y: row })
    }

    fn handle_event(&mut self, event: &Event) -> Option<FocusableEvent> {
        if self.disabled {
            return None;
        }

        if let Event::Key(key_event) = event {
            match key_event.code {
                KeyCode::Char(c) => {
                    self.value.push(c);
                    Some(FocusableEvent::Input(self.value.to_string()))
                }
                KeyCode::Backspace => {
                    self.value.pop();
                    Some(FocusableEvent::Input(self.value.to_string()))
                }
                KeyCode::Enter => Some(FocusableEvent::Enter),
                _ => None,
            }
        } else {
            None
        }
    }
}
