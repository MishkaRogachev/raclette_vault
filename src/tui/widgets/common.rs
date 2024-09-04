
use ratatui::{
    crossterm::event::{Event, KeyCode, KeyEvent, MouseButton, MouseEvent, MouseEventKind},
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame
};

pub trait Widget {
    fn handle_event(&mut self, event: Event) -> Option<Event>;
    fn draw(&mut self, frame: &mut Frame, area: ratatui::layout::Rect);
}

pub struct Button {
    label: String,
    hotkey: Option<char>,
    action: Option<Box<dyn Fn() + Send>>,
    is_hovered: bool,
    is_pressed: bool,
    area: Option<Rect>,
}

impl Button {
    pub fn new(label: &str, hotkey: Option<char>) -> Self {
        Button {
            label: label.to_string(),
            hotkey,
            action: None,
            is_hovered: false,
            is_pressed: false,
            area: None,
        }
    }

    pub fn action<F: Fn() + 'static + Send>(mut self, f: F) -> Self {
        self.action = Some(Box::new(f));
        self
    }

    fn contains(&self, x: u16, y: u16) -> bool {
        if let Some(area) = self.area {
            return area.x <= x && x < area.x + area.width && area.y <= y && y < area.y + area.height;
        }
        false
    }

    fn handle_mouse_event(&mut self, mouse_event: MouseEvent) -> Option<Event> {
        self.is_hovered = self.contains(mouse_event.column, mouse_event.row);
        self.is_pressed = mouse_event.kind == MouseEventKind::Down(MouseButton::Left);

        if self.is_hovered && self.is_pressed {
            if let Some(action) = &self.action {
                action();
                return None;
            }
        }
        Some(Event::Mouse(mouse_event))
    }

    fn handle_key_event(&self, key_event: KeyEvent) -> Option<Event> {
        if let Some(hotkey) = self.hotkey {
            if key_event.code == KeyCode::Char(hotkey) {
                if let Some(action) = &self.action {
                    action();
                    return None;
                }
            }
        }
        Some(Event::Key(key_event))
    }
}

impl Widget for Button {
    fn handle_event(&mut self, event: Event) -> Option<Event> {
        match event {
            Event::Mouse(mouse_event) => self.handle_mouse_event(mouse_event),
            Event::Key(key_event) => self.handle_key_event(key_event),
            _ => Some(event),
        }
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) {
        self.area = Some(area); // Store the button's area for later use

        let block = Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::Yellow));
        let style = Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD);
        
        let (block, style) = if self.is_hovered {
            (
                block,
                style.bg(Color::Yellow).fg(Color::Black),
            )
        } else {
            (block, style,)
        };

        let label_line = if let Some(hotkey) = self.hotkey {
            let mut spans = Vec::new();
            let mut found = false;

            for c in self.label.chars() {
                if !found && c.to_ascii_lowercase() == hotkey.to_ascii_lowercase() {
                    // Apply underline modifier to the matching character
                    spans.push(Span::styled(
                        c.to_string(),
                        style.add_modifier(Modifier::UNDERLINED),
                    ));
                    found = true;
                } else {
                    spans.push(Span::styled(c.to_string(), style));
                }
            }
            Line::from(spans)
        } else {
            Line::from(Span::styled(self.label.to_string(), style))
        };

        let paragraph = Paragraph::new(label_line)
            .block(block)
            .alignment(ratatui::layout::Alignment::Center);

        frame.render_widget(paragraph, area);
    }
}
