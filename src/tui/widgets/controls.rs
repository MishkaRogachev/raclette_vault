use std::{borrow::BorrowMut, collections::HashMap};

use ratatui::{
    crossterm::event::{Event, KeyCode, KeyEvent, MouseButton, MouseEventKind},
    layout::{Alignment, Constraint, Direction, Layout, Position, Rect},
    style::{Color, Modifier, Style, Stylize},
    symbols,
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Gauge, Paragraph, Widget},
    Frame
};
use zeroize::Zeroizing;

pub const BUTTONS_HEIGHT: u16 = 3;
pub const SWITCH_HEIGHT: u16 = 3;
pub const CHECKBOX_HEIGHT: u16 = 3;
pub const CHECKBOX_WIDTH: u16 = 5;

const CHECKMARK_ON: &str = "x";
const CHECKMARK_OFF: &str = " ";

const BUSY_PERIOD: u128 = 250;
const BUSY_SYMBOLS: [char; 7] = ['▚', '▞', '▘', '▝', '▗', '▖', '▘'];

pub enum FocusableEvent {
    FocusChanged,
    FocusFinished,
    Input(String),
    Enter,
}

pub trait Focusable {
    fn is_focused(&self) -> bool;
    fn set_focused(&mut self, focused: bool);
    fn contains(&self, column: u16, row: u16) -> bool;
    fn handle_event(&mut self, event: &Event) -> Option<FocusableEvent>;
}

pub struct Button {
    pub label: String,
    pub hotkey: Option<char>,
    pub disabled: bool,
    pub active: bool,
    pub color: Color,
    pub is_hovered: bool,
    area: Rect,
}

pub struct MultiSwitch {
    pub options: Vec<Button>,
    pub active_index: usize,
}

pub struct SwapButton {
    pub state: bool,
    pub first: Button,
    pub second: Button,
}

pub struct MenuButton {
    pub button: Button,
    pub options: Vec<Button>,
    pub is_open: bool
}

pub struct ProgressBar {
    pub min: u64,
    pub max: u64,
    pub value: u64,
    pub color: Color,
}

pub struct Input {
    pub value: Zeroizing<String>,
    pub placeholder: String,
    pub disabled: bool,
    pub color: Color,
    pub focused: bool,
    pub masked: bool,
    area: Rect,
}

pub struct CheckBox {
    pub label: String,
    pub hotkey: Option<char>,
    pub disabled: bool,
    pub toggled: bool,
    pub color: Color,
    pub is_hovered: bool,
    area: Rect,
}

pub struct CheckList<T> {
    pub options: HashMap<T, CheckBox>
}

pub struct Busy {
    label: String,
    start_time: tokio::time::Instant,
}

impl Button {
    pub fn new(label: &str, hotkey: Option<char>) -> Self {
        Button {
            label: label.to_string(),
            hotkey,
            disabled: false,
            active: false,
            color: Color::Yellow,
            is_hovered: false,
            area: Rect::default(),
        }
    }

    pub fn warning(mut self) -> Self {
        self.color = Color::Red;
        self
    }

    pub fn primary(mut self) -> Self {
        self.color = Color::White;
        self
    }

    pub fn disable(mut self) -> Self {
        self.disabled = true;
        self
    }

    pub fn handle_event(&mut self, event: &Event) -> Option<()> {
        if self.disabled {
            return None;
        }

        match event {
            Event::Mouse(mouse_event) => {
                self.is_hovered = self.area.contains(Position { x: mouse_event.column, y: mouse_event.row });
                if self.is_hovered && mouse_event.kind == MouseEventKind::Down(MouseButton::Left) {
                    return Some(());
                }
                None
            },
            Event::Key(key_event) => {
                if let Some(hotkey) = self.hotkey {
                    if key_event.code == KeyCode::Char(hotkey) {
                        return Some(());
                    }
                }
                None
            },
            _ => None
        }
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        self.area = area; // Store the button's area for mouse handling

        let mut style = Style::default().fg(self.color).add_modifier(Modifier::BOLD);
        let mut block = Block::default().borders(Borders::ALL).border_style(style);

        if self.disabled {
            block = block.dim();
        }

        if !self.disabled && self.is_hovered {
            block = block.border_set(symbols::border::DOUBLE);
            style = style.bg(self.color).fg(Color::Black);
        }

        // active overrides hovered
        if self.active {
            block = block.borders(Borders::ALL)
                .border_set(symbols::border::THICK);
        }

        let label_line = render_label_with_hotkey(&self.label, self.hotkey, style);
        let paragraph = Paragraph::new(label_line)
            .block(block)
            .alignment(ratatui::layout::Alignment::Center);
        frame.render_widget(paragraph, area);
    }
}

impl MultiSwitch {
    pub fn new(mut options: Vec<Button>) -> Self {

        let active_index = 0;
        if options.len() > 0 {
            options[active_index].active = true;
        }

        Self { options, active_index }
    }

    pub fn handle_event(&mut self, event: &Event) -> Option<usize> {
        for (i, button) in self.options.iter_mut().enumerate() {
            if let Some(()) = button.handle_event(event) {
                self.set_active(i);
                return Some(i);
            }
        }
        None
    }

    pub fn set_active(&mut self, index: usize) {
        if index < self.options.len() {
            if let Some(button) = self.options.get_mut(self.active_index) {
                button.active = false;
            }

            if let Some(button) = self.options.get_mut(index) {
                button.active = true;
                self.active_index = index;
            }
        } else if self.options.len() > 0 {
            self.set_active(0)
        }
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        let button_width = area.width / self.options.len() as u16;
        let buttons_area = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                vec![Constraint::Length(button_width as u16); self.options.len()]
            )
            .margin(0)
            .split(area);

        for (i, button) in self.options.iter_mut().enumerate() {
            button.render(frame, buttons_area[i]);
        }
    }
}

impl SwapButton {
    pub fn new(first: Button, second: Button) -> Self {
        Self {
            state: false,
            first,
            second,
        }
    }

    pub fn swap(&mut self) -> bool {
        self.state = !self.state;
        self.state
    }

    pub fn handle_event(&mut self, event: &Event) -> Option<bool> {
        let button = if self.state { &mut self.second } else { &mut self.first };
        if let Some(()) = button.handle_event(event) {
            Some(self.swap())
        } else {
            None
        }
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        if self.state {
            self.second.render(frame, area);
        } else {
            self.first.render(frame, area);
        }
    }
}

impl MenuButton {
    pub fn new(label: &str, hotkey: Option<char>, options: Vec<Button>) -> Self {
        let button = Button::new(label, hotkey);
        Self { button, options, is_open: false }
    }

    pub fn handle_event(&mut self, event: &Event) -> Option<usize> {
        if self.is_open {
            for (i, option) in self.options.iter_mut().enumerate() {
                if let Some(()) = option.handle_event(event) {
                    self.is_open = false;
                    return Some(i);
                }
            }

            if let Event::Mouse(mouse_event) = event {
                if mouse_event.kind == MouseEventKind::Down(MouseButton::Left) {
                    self.is_open = false;
                    return None;
                }
            } else if event == &Event::Key(KeyCode::Esc.into()) {
                self.is_open = false;
                return None;
            }
        } else if let Some(()) = self.button.handle_event(event) {
            self.is_open = true;
        }
        None
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        self.button.render(frame, area);

        if self.is_open {
            let menu_height = self.options.len() as u16 * BUTTONS_HEIGHT;
            let menu_area = Rect {
                x: area.x,
                y: area.y - menu_height,
                width: area.width,
                height: menu_height,
            };
            frame.render_widget(Clear, menu_area);

            let background_block = Block::default().style(Style::default().bg(Color::Black)).borders(Borders::ALL);
            frame.render_widget(background_block, menu_area);

            let options_area = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![Constraint::Length(BUTTONS_HEIGHT); self.options.len()])
                .split(menu_area);

            for (i, option) in self.options.iter_mut().enumerate() {
                option.render(frame, options_area[i]);
            }
        }
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

impl ProgressBar {
    pub fn new(min: u64, max: u64, value: u64) -> Self {
        Self {
            min,
            max,
            value,
            color: Color::Yellow,
        }
    }

    #[allow(dead_code)]
    pub fn set_value(&mut self, value: u64) {
        self.value = value.min(self.max).max(self.min);
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        let ratio = (self.value - self.min) as f64 / (self.max - self.min) as f64;
        let style = Style::default().fg(self.color);
        Gauge::default()
            .block(Block::bordered().style(style))
            .gauge_style(style)
            .label(format!("{}/{}", self.value + 1, self.max))
            .ratio(ratio).render(area, frame.buffer_mut());
    }
}

pub fn handle_scoped_event(focusables: &mut [&mut dyn Focusable], event: &Event) -> Option<FocusableEvent> {
    // Find the focused widget
    let mut focused_index: Option<usize> = None;
    for (i, widget) in focusables.iter_mut().enumerate() {
        if widget.is_focused() {
            focused_index = Some(i);
            break;
        }
    }

    match event {
        Event::Key(KeyEvent { code: KeyCode::Tab, .. }) => {
            if let Some(index) = focused_index {
                // Unfocus the current widget
                focusables[index].set_focused(false);

                // If focused widget is last, unfocus it (no focus)
                if index + 1 >= focusables.len() {
                    return Some(FocusableEvent::FocusChanged);
                }

                // Focus the next widget
                focusables[index + 1].set_focused(true);
            } else {
                // If no focused widget, focus the first one
                if !focusables.is_empty() {
                    focusables[0].set_focused(true);
                }
            }
            return Some(FocusableEvent::FocusChanged);
        }

        // If esc is pressed, unfocus the current widget
        Event::Key(KeyEvent { code: KeyCode::Esc, .. }) => {
            if let Some(index) = focused_index {
                focusables[index].set_focused(false); // Unfocus the current widget
            }
            return Some(FocusableEvent::FocusChanged);
        }

        // If mouse click is pressed, focus the widget that was clicked
        Event::Mouse(mouse_event) => {
            if mouse_event.kind == MouseEventKind::Down(MouseButton::Left) {
                for widget in focusables.iter_mut() {
                    widget.set_focused(widget.contains(mouse_event.column, mouse_event.row));
                }
            }
        }
        _ => {}
    }

    // If there is a focused widget, call handle_event on it
    if let Some(index) = focused_index {
        if let Some(event) = focusables[index].handle_event(event) {
            match event {
                FocusableEvent::Enter => {
                    // Enter is pressed, unfocus the current widget and focus the next one
                    focusables[index].set_focused(false);
                    if index + 1 < focusables.len() {
                        focusables[index + 1].set_focused(true);
                        return Some(FocusableEvent::FocusChanged);
                    }
                    return Some(FocusableEvent::FocusFinished);
                }
                _ => { return Some(event); }
            }
        }
    }
    return None;
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

    #[allow(dead_code)]
    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }

    pub fn warning(mut self) -> Self {
        self.color = Color::Red;
        self
    }

    #[allow(dead_code)]
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


impl<T> CheckList<T>
where T: Eq + std::hash::Hash + Clone {
    pub fn new(options: HashMap<T, CheckBox>) -> Self {
        Self { options }
    }

    #[allow(dead_code)]
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

impl Busy {
    pub fn new(label: &str) -> Self {
        Self {
            label: label.to_string(),
            start_time: tokio::time::Instant::now(),
        }
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        let elapsed = self.start_time.elapsed().as_millis();
        let index = (elapsed / BUSY_PERIOD as u128 % BUSY_SYMBOLS.len() as u128) as usize;
        let symbol = BUSY_SYMBOLS[index];
        let text = format!("{} {}", self.label, symbol);

        Paragraph::new(text)
            .style(Style::default().fg(Color::Yellow))
            .alignment(Alignment::Right)
            .render(area, frame.buffer_mut());
    }
}