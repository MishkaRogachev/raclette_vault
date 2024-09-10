use ratatui::{
    crossterm::event::{Event, KeyCode, KeyEvent},
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame
};
use regex::Regex;

use super::common::{Control, Widget};

pub struct Input {
    last_value: String,
    value: String,
    pub placeholder: String,
    pub color: Color,
    pub focused: bool,
    pub mask: Option<String>,
    pub validator: Option<Regex>,
    pub control: Control,
    pub on_enter: Option<Box<dyn Fn(String) + Send>>,
}

impl Input {
    pub fn new(placeholder: &str) -> Self {
        Self {
            last_value: String::new(),
            value: String::new(),
            placeholder: placeholder.to_string(),
            color: Color::Yellow,
            focused: false,
            mask: None,
            validator: None,
            control: Control::new(),
            on_enter: None,
        }
    }

    pub fn masked(mut self, mask: &str) -> Self {
        self.mask = Some(mask.to_string());
        self
    }

    pub fn value(mut self, value: &str) -> Self {
        self.last_value = value.to_string();
        self.value = value.to_string();
        self
    }

    pub fn on_enter<F: Fn(String) + 'static + Send>(mut self, callback: F) -> Self {
        self.on_enter = Some(Box::new(callback));
        self
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) -> Option<Event> {
        match key_event.code {
            KeyCode::Char(c) => {
                self.value.push(c);
                None
            }
            KeyCode::Backspace => {
                self.value.pop();
                None
            }
            KeyCode::Enter => {
                if let Some(ref regex) = self.validator {
                    if regex.is_match(&self.value) {
                        if let Some(func) = &self.on_enter {
                            func(self.value.clone());
                        }
                    }
                } else if let Some(func) = &self.on_enter {
                    func(self.value.clone());
                }
                self.last_value = self.value.clone();
                self.focused = false;
                None
            }
            KeyCode::Esc => {
                self.value = self.last_value.clone();
                self.focused = false;
                None
            }
            _ => Some(Event::Key(key_event)),
        }
    }
}

impl Widget for Input {
    fn handle_event(&mut self, event: Event) -> Option<Event> {
        match event {
            Event::Key(key_event) => self.handle_key_event(key_event),
            _ => Some(event),
        }
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) {
        self.control.area = Some(area);

        let style = if self.focused {
            Style::default().fg(Color::Black).bg(self.color)
        } else {
            Style::default().fg(self.color)
        };

        let block = Block::default().borders(Borders::ALL).border_style(style);
        let display_value = if self.value.is_empty() {
            Span::styled(&self.placeholder, Style::default().fg(Color::DarkGray))
        } else {
            if let Some(mask) = &self.mask {
                Span::styled(mask.repeat(self.value.len()), style)
            } else {
                Span::styled(&self.value, style)
            }
        };

        let paragraph = Paragraph::new(Line::from(display_value)).block(block);
        frame.render_widget(paragraph, area);
    }
}
