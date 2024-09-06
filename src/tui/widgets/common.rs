
use ratatui::{
    crossterm::event::{Event, KeyCode, KeyEvent, MouseButton, MouseEvent, MouseEventKind},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame
};

pub trait Widget {
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
    pub off_label: String,
    pub on_label: String,
    pub hotkey: Option<char>,
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

    pub fn primary(mut self) -> Self {
        self.color = Color::White;
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
            if let Some(func) = &self.on_down {
                func();
                return None;
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

        let label_line =  render_label_with_hotkey(&self.label, self.hotkey, style);
        let paragraph = Paragraph::new(label_line)
            .block(block)
            .alignment(ratatui::layout::Alignment::Center);
        frame.render_widget(paragraph, area);
    }
}

impl Switch {
    pub fn new(off_label: &str, on_label: &str, hotkey: Option<char>) -> Self {
        Switch {
            off_label: off_label.to_string(),
            on_label: on_label.to_string(),
            hotkey,
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
        if let Some(hotkey) = self.hotkey {
            if key_event.code == KeyCode::Char(hotkey) {
                self.is_on = !self.is_on;
                if let Some(func) = &self.on_toggle {
                    func(self.is_on);
                }
                return None;
            }
        } else {
            return Some(Event::Key(key_event))
        };
        Some(Event::Key(key_event))
    }
}

impl Widget for Switch {
    fn handle_event(&mut self, event: Event) -> Option<Event> {
        match event {
            Event::Mouse(mouse_event) => self.handle_mouse_event(mouse_event),
            Event::Key(key_event) => self.handle_key_event(key_event),
            _ => Some(event),
        }
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) {
        self.control.area = Some(area);

        let active_style = Style::default().fg(Color::Black).bg(Color::Yellow).add_modifier(Modifier::BOLD);
        let inactive_style = Style::default().fg(Color::White);

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Yellow));
        let inner_area = block.inner(area);

        frame.render_widget(block, area);

        let column_constraints = [
            Constraint::Percentage(49),
            Constraint::Length(1),
            Constraint::Percentage(49),
        ];

        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(column_constraints)
            .split(inner_area);

        let off_area = layout[0];
        let separator_area = layout[1];
        let on_area = layout[2];

        // Render the "OFF" and "ON" labels with respective styles
        let off_paragraph = Paragraph::new(render_label_with_hotkey(
            &self.off_label,
            self.hotkey,
            if !self.is_on { active_style } else { inactive_style },
        ))
        .alignment(ratatui::layout::Alignment::Center)
        .block(Block::default().borders(Borders::NONE));

        let on_paragraph = Paragraph::new(render_label_with_hotkey(
            &self.on_label,
            self.hotkey,
            if self.is_on { active_style } else { inactive_style },
        ))
        .alignment(ratatui::layout::Alignment::Center)
        .block(Block::default().borders(Borders::NONE));

        // Render the separator in the center
        let separator_paragraph = Paragraph::new("|")
            .alignment(ratatui::layout::Alignment::Center)
            .style(Style::default().fg(Color::Yellow))
            .block(Block::default().borders(Borders::NONE));

        // Render the OFF label, separator, and ON label in the appropriate areas
        frame.render_widget(off_paragraph, off_area);
        frame.render_widget(separator_paragraph, separator_area);
        frame.render_widget(on_paragraph, on_area);
    }
}

pub fn render_label_with_hotkey(label: &str, hotkey: Option<char>, style: Style) -> Line<'_> {
    if let Some(hotkey) = hotkey {
        let mut spans = Vec::new();
        let mut found = false;

        for c in label.chars() {
            if !found && c.to_ascii_lowercase() == hotkey.to_ascii_lowercase() {
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
        Line::from(Span::styled(label.to_string(), style))
    }
}
