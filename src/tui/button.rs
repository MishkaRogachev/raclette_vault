
use ratatui::{
    buffer::Buffer,
    crossterm::event::{Event, KeyCode, KeyEvent, MouseButton, MouseEvent, MouseEventKind},
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Widget}
};

pub struct Button {
    label: String,
    hotkey: Option<char>,
    action: Option<Box<dyn Fn()>>,
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

    pub fn handle_event(&mut self, event: Option<Event>) -> Option<Event> {
        if let Some(event) = event {
            match event {
                Event::Mouse(mouse_event) => {
                    self.handle_mouse_event(mouse_event);
                    return None;
                },
                Event::Key(key_event) => {
                    self.handle_key_event(key_event);
                    return None;
                },
                _ => { return Some(event); },
            };
        } else {
            return event;
        }

    }

    pub fn action<F: Fn() + 'static>(mut self, f: F) -> Self {
        self.action = Some(Box::new(f));
        self
    }

    fn handle_mouse_event(&mut self, mouse_event: MouseEvent) {
        // Debug the mouse event and button area
        //println!("MouseEvent at ({}, {}), Button area: {:?}", mouse_event.column, mouse_event.row, self.area);

        self.is_hovered = mouse_event.kind == MouseEventKind::Moved
            && self.contains(mouse_event.column, mouse_event.row);
        self.is_pressed = mouse_event.kind == MouseEventKind::Down(MouseButton::Left)
            && self.contains(mouse_event.column, mouse_event.row);

        if self.is_hovered && mouse_event.kind == MouseEventKind::Up(MouseButton::Left) {
            println!("Button clicked!");
            if let Some(action) = &self.action {
                action();
            }
        }
    }

    fn handle_key_event(&self, key_event: KeyEvent) {
        if let Some(hotkey) = self.hotkey {
            if key_event.code == KeyCode::Char(hotkey) {
                if let Some(action) = &self.action {
                    action();
                }
            }
        }
    }

    fn contains(&self, x: u16, y: u16) -> bool {
        if let Some(area) = self.area {
            return area.x <= x && x < area.x + area.width && area.y <= y && y < area.y + area.height;
        }
        false
    }
}

impl Widget for Button {
    fn render(mut self, area: Rect, buf: &mut Buffer) {
        self.area = Some(area); // Store the button's area for later use

        let block = Block::default().borders(Borders::ALL);
        let inner_area = block.inner(area);

        let style = if self.is_pressed {
            Style::default()
                .fg(Color::Black)
                .bg(Color::Yellow)
                .add_modifier(Modifier::BOLD | Modifier::REVERSED)
        } else if self.is_hovered {
            Style::default()
                .fg(Color::Black)
                .bg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        };

        block
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Yellow))
            .render(area, buf);

        buf.set_string(inner_area.x, inner_area.y, &self.label, style);
    }
}
