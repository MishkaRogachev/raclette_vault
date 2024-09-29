use std::collections::HashMap;
use ratatui::{
    crossterm::event::{Event, KeyCode, MouseButton, MouseEventKind},
    layout::{Alignment, Position, Rect},
    style::{Color, Modifier, Style, Stylize},
    widgets::{Block, Borders, Paragraph},
    Frame
};

pub const CHECKBOX_HEIGHT: u16 = 3;
pub const CHECKBOX_WIDTH: u16 = 5;

const CHECKMARK_ON: &str = "x";
const CHECKMARK_OFF: &str = " ";

pub struct CheckBox {
    pub label: String,
    pub hotkey: Option<char>,
    pub disabled: bool,
    pub toggled: bool,
    pub color: Color,
    pub is_hovered: bool,
    area: Rect,
}


pub struct CheckOptions<T> {
    pub options: HashMap<T, CheckBox>
}

impl CheckBox {
    pub fn new(label: &str, toggled: bool, hotkey: Option<char>) -> Self {
        CheckBox {
            label: label.to_string(),
            hotkey,
            disabled: false,
            toggled,
            color: Color::Yellow,
            is_hovered: false,
            area: Rect::default(),
        }
    }

    pub fn disabled(mut self) -> Self {
        self.disabled = true;
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

    pub fn handle_event(&mut self, event: &Event) -> Option<bool> {
        if self.disabled {
            return None;
        }

        match event {
            Event::Mouse(mouse_event) => {
                self.is_hovered = self.area.contains(Position { x: mouse_event.column, y: mouse_event.row });
                if self.is_hovered && mouse_event.kind == MouseEventKind::Down(MouseButton::Left) {
                    self.toggled = !self.toggled;
                    return Some(self.toggled);
                }
                None
            },
            Event::Key(key_event) => {
                if let Some(hotkey) = self.hotkey {
                    if key_event.code == KeyCode::Char(hotkey) {
                        self.toggled = !self.toggled;
                        return Some(self.toggled);
                    }
                }
                None
            },
            _ => None
        }
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        self.area = area; // Store the checkbox's area for mouse handling

        let checkmark = if self.toggled { CHECKMARK_ON } else { CHECKMARK_OFF };
        let mut style = Style::default().fg(self.color).add_modifier(Modifier::BOLD);

        if self.disabled {
            style = style.dim();
        }

        if !self.disabled && self.is_hovered {
            style = style.bg(self.color).fg(Color::Black);
        }

        let checkmark_block = Block::default()
            .borders(Borders::ALL)
            .style(style);

        let checkmark_paragraph = Paragraph::new(checkmark)
            .block(checkmark_block)
            .alignment(Alignment::Center);

        let checkmark_area = Rect {
            x: area.x,
            y: area.y,
            width: CHECKBOX_WIDTH,
            height: area.height,
        };

        frame.render_widget(checkmark_paragraph, checkmark_area);

        let label_area = Rect {
            x: checkmark_area.x + checkmark_area.width + 1, // Place the label to the right of the checkbox
            y: area.y + CHECKBOX_HEIGHT / 2, // Center the label vertically
            width: area.width.saturating_sub(checkmark_area.width + 1),
            height: area.height,
        };

        let label_paragraph = Paragraph::new(self.label.clone())
            .style(Style::default().fg(self.color))
            .alignment(Alignment::Left);

        frame.render_widget(label_paragraph, label_area);
    }
}


impl<T> CheckOptions<T>
where T: Eq + std::hash::Hash + Clone {
    pub fn new(options: HashMap<T, CheckBox>) -> Self {
        Self { options }
    }

    pub fn option(&mut self, key: T) -> Option<&mut CheckBox> {
        self.options.get_mut(&key)
    }

    pub fn toggle_by_keys(&mut self, keys: &[T]) {
        for (key, checkbox) in self.options.iter_mut() {
            checkbox.toggled = keys.contains(key);
        }
    }

    pub fn get_checked_keys(&self) -> Vec<T> {
        self.options.iter()
            .filter_map(|(key, checkbox)| if checkbox.toggled { Some(key.clone()) } else { None })
            .collect()
    }

    pub fn handle_event(&mut self, event: &Event) -> Option<(T, bool)> {
        for (key, checkbox) in self.options.iter_mut() {
            if let Some(toggled) = checkbox.handle_event(event) {
                return Some((key.clone(), toggled));
            }
        }
        None
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        let mut y_offset = area.y;

        for (_, checkbox) in &mut self.options {
            let checkbox_area = Rect {
                x: area.x,
                y: y_offset,
                width: area.width,
                height: CHECKBOX_HEIGHT,
            };
            checkbox.render(frame, checkbox_area);
            y_offset += CHECKBOX_HEIGHT;
        }
    }
}