
use ratatui::{
    crossterm::event::{Event, KeyCode, KeyEvent, MouseButton, MouseEvent, MouseEventKind},
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame
};

pub trait ControlTrait {
    fn handle_event(&mut self, event: Event) -> Option<Event>;
    fn draw(&mut self, frame: &mut Frame, area: ratatui::layout::Rect);
}

pub struct Control {
    pub is_hovered: bool,
    pub is_down: bool,
    pub area: Option<Rect>,
}

pub struct Button {
    pub label: String,
    pub hotkey: Option<char>,
    pub color: Color,
    pub on_up: Option<Box<dyn Fn() + Send>>,
    pub on_down: Option<Box<dyn Fn() + Send>>,
    pub control: Control,
}

pub struct Switch {
    pub on_label: String,
    pub off_label: String,
    pub is_on: bool,
    pub on_toggle: Option<Box<dyn Fn(bool) + Send>>,
    pub control: Control,
}

impl Control {
    pub fn new() -> Self {
        Control {
            is_hovered: false,
            is_down: false,
            area: None,
        }
    }

    fn contains(&self, x: u16, y: u16) -> bool {
        if let Some(area) = self.area {
            return area.x <= x && x < area.x + area.width && area.y <= y && y < area.y + area.height;
        }
        false
    }

    fn handle_mouse_hover(&mut self, mouse_event: MouseEvent) {
        self.is_hovered = self.contains(mouse_event.column, mouse_event.row);
    }
}

impl Button {
    pub fn new(label: &str, hotkey: Option<char>) -> Self {
        Button {
            label: label.to_string(),
            hotkey,
            color: Color::Yellow,
            on_up: None,
            on_down: None,
            control: Control::new(),
        }
    }
    
    pub fn on_up<F: Fn() + 'static + Send>(mut self, f: F) -> Self {
        self.on_up = Some(Box::new(f));
        self
    }

    pub fn on_down<F: Fn() + 'static + Send>(mut self, f: F) -> Self {
        self.on_down = Some(Box::new(f));
        self
    }

    pub fn warning(mut self) -> Self {
        self.color = Color::Red;
        self
    }

    fn handle_mouse_event(&mut self, mouse_event: MouseEvent) -> Option<Event> {
        self.control.is_hovered = self.control.contains(mouse_event.column, mouse_event.row);
        if self.control.is_down && mouse_event.kind == MouseEventKind::Up(MouseButton::Left) {
            self.control.is_down = false;
            if let Some(func) = &self.on_up {
                func();
                return None;
            }
        } else if self.control.is_hovered && mouse_event.kind == MouseEventKind::Down(MouseButton::Left) {
            if let Some(func) = &self.on_down {
                self.control.is_down = true;
                func();
                return None;
            }
        }
        Some(Event::Mouse(mouse_event))
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) -> Option<Event> {
        let hotkey = if let Some(hotkey) = self.hotkey {
            hotkey 
        } else {
            return Some(Event::Key(key_event))
        };

        if key_event.code == KeyCode::Char(hotkey) {
            if self.control.is_down {
                self.control.is_down = false;
                if let Some(func) = &self.on_up {
                    func();
                    return None;
                }
            } else {
                self.control.is_down = true;
                if let Some(func) = &self.on_down {
                    func();
                    return None;
                }
            }
        }
        Some(Event::Key(key_event))
    }
}

impl ControlTrait for Button {
    fn handle_event(&mut self, event: Event) -> Option<Event> {
        match event {
            Event::Mouse(mouse_event) => self.handle_mouse_event(mouse_event),
            Event::Key(key_event) => self.handle_key_event(key_event),
            _ => Some(event),
        }
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) {
        self.control.area = Some(area); // Store the button's area for later use

        let style = Style::default().fg(self.color).add_modifier(Modifier::BOLD);
        let block = Block::default().borders(Borders::ALL).border_style(style);

        let (block, style) = if self.control.is_hovered {
            (
                block,
                style.bg(self.color).fg(Color::Black),
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

impl Switch {
    pub fn new(on_label: &str, off_label: &str) -> Self {
        Switch {
            on_label: on_label.to_string(),
            off_label: off_label.to_string(),
            is_on: false,
            on_toggle: None,
            control: Control::new(),
        }
    }

    pub fn on_toggle<F: Fn(bool) + 'static + Send>(mut self, f: F) -> Self {
        self.on_toggle = Some(Box::new(f));
        self
    }

    fn handle_mouse_event(&mut self, mouse_event: MouseEvent) -> Option<Event> {
        self.control.handle_mouse_hover(mouse_event);

        if self.control.is_hovered && mouse_event.kind == MouseEventKind::Down(MouseButton::Left) {
            self.is_on = !self.is_on;
            if let Some(func) = &self.on_toggle {
                func(self.is_on);
            }
            return None;
        }

        Some(Event::Mouse(mouse_event))
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) -> Option<Event> {
        if matches!(key_event.code, KeyCode::Char(' ') | KeyCode::Enter) {
            self.is_on = !self.is_on;
            if let Some(func) = &self.on_toggle {
                func(self.is_on);
            }
            return None;
        }
        Some(Event::Key(key_event))
    }
}

impl ControlTrait for Switch {
    fn handle_event(&mut self, event: Event) -> Option<Event> {
        match event {
            Event::Mouse(mouse_event) => self.handle_mouse_event(mouse_event),
            Event::Key(key_event) => self.handle_key_event(key_event),
            _ => Some(event),
        }
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) {
        self.control.area = Some(area);

        let half_width = area.width / 2;
        let on_area = Rect {
            x: area.x,
            y: area.y,
            width: half_width,
            height: area.height,
        };
        let off_area = Rect {
            x: area.x + half_width,
            y: area.y,
            width: area.width - half_width,
            height: area.height,
        };

        let active_style = Style::default().fg(Color::Black).bg(Color::Yellow).add_modifier(Modifier::BOLD);
        let inactive_style = Style::default().fg(Color::White);

        let on_paragraph = Paragraph::new(self.on_label.clone())
            .block(Block::default().borders(Borders::ALL))
            .style(if !self.is_on { active_style } else { inactive_style })
            .alignment(ratatui::layout::Alignment::Center);

        let off_paragraph = Paragraph::new(self.off_label.clone())
            .block(Block::default().borders(Borders::ALL))
            .style(if self.is_on { active_style } else { inactive_style })
            .alignment(ratatui::layout::Alignment::Center);

        frame.render_widget(on_paragraph, on_area);
        frame.render_widget(off_paragraph, off_area);
    }
}
