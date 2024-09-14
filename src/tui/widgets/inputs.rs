use std::sync::{atomic::AtomicBool, Arc};
use zeroize::Zeroizing;
use ratatui::{
    crossterm::event::{Event, KeyCode, KeyEvent},
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame
};


use super::{common::{Control, Widget}, focus::Focusable};

pub struct Input {
    last_value: Zeroizing<String>,
    pub value: Zeroizing<String>,
    pub placeholder: String,
    pub color: Color,
    pub focused: bool,
    mask_flag: Option<Arc<AtomicBool>>,
    control: Control,
    on_input: Option<Box<dyn Fn(&str) + Send>>,
}

impl Input {
    pub fn new(placeholder: &str) -> Self {
        Self {
            last_value: Zeroizing::new(String::new()),
            value: Zeroizing::new(String::new()),
            placeholder: placeholder.to_string(),
            color: Color::Yellow,
            focused: false,
            mask_flag: None,
            control: Control::new(),
            on_input: None,
        }
    }

    pub fn mask(mut self, mask: Arc<AtomicBool>) -> Self {
        self.mask_flag = Some(mask);
        self
    }

    pub fn set_value(mut self, value: &str) -> Self {
        *self.last_value = value.to_string();
        *self.value = value.to_string();
        self
    }

    pub fn on_input<F: Fn(&str) + 'static + Send>(mut self, callback: F) -> Self {
        self.on_input = Some(Box::new(callback));
        self
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) -> Option<Event> {
        match key_event.code {
            KeyCode::Char(c) => {
                self.value.push(c);
            }
            KeyCode::Backspace => {
                self.value.pop();
            }
            KeyCode::Enter => {
                self.last_value = self.value.clone();
                self.focused = false;
            }
            KeyCode::Esc | KeyCode::Enter => {
                self.value = self.last_value.clone();
                self.focused = false;
            }
            _ => return Some(Event::Key(key_event)),
        };
        if let Some(func) = &self.on_input {
            let value: &str = &self.value;
            func(value);
        }
        None
    }
}

impl Widget for Input {
    fn handle_event(&mut self, event: Event) -> Option<Event> {
        match event {
            Event::Key(key_event) => self.handle_key_event(key_event),
            _ => Some(event),
        }
    }

    fn process(&mut self, frame: &mut Frame, area: Rect) {
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
            if self.mask_flag.is_some() && !self.mask_flag.as_ref().unwrap().load(std::sync::atomic::Ordering::Relaxed) {
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
        self.control.contains(column, row)
    }
}
