use std::collections::HashMap;

use ratatui::{
    crossterm::event::{Event, KeyCode, KeyEvent, MouseButton, MouseEvent, MouseEventKind},
    layout::{Alignment, Constraint, Direction, Layout, Margin, Position, Rect},
    style::{Color, Modifier, Style, Stylize},
    symbols,
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Gauge, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState, Widget},
    Frame
};
use regex::Regex;
use zeroize::Zeroizing;

pub const BUTTON_HEIGHT: u16 = 3;
pub const SWITCH_HEIGHT: u16 = 3;
pub const CHECKBOX_HEIGHT: u16 = 3;
pub const CHECKBOX_WIDTH: u16 = 5;
pub const INPUT_HEIGHT: u16 = 3;

const CHECKMARK_ON: &str = "x";
const CHECKMARK_OFF: &str = " ";

const BUSY_PERIOD: u128 = 250;
const BUSY_SYMBOLS: [char; 8] = ['🌑', '🌒', '🌓', '🌔', '🌕', '🌖', '🌗', '🌘'];

const SCROLL_UP_SYMBOL: &str = "⌃";
const SCROLL_DOWN_SYMBOL: &str = "⌄";
const SCROLL_THUMB_SYMBOL: &str = "┃";

pub enum InputEvent {
    FocusChanged,
    FocusFinished,
    Input(String),
    ValidationFailed,
    Enter,
}

pub enum MenuEvent<T> {
    Selected(T),
    Opened,
    Closed
}

pub trait Focusable {
    fn is_focused(&self) -> bool;
    fn set_focused(&mut self, focused: bool);
    fn contains(&self, column: u16, row: u16) -> bool;
    fn handle_event(&mut self, event: &Event) -> Option<InputEvent>;
}

pub struct Button {
    pub label: String,
    pub hotkey: Option<char>,
    pub disabled: bool,
    pub active: bool,
    pub color: Color,
    pub is_hovered: bool,
    pub default: bool,
    pub escape: bool,
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
    regex: Option<Regex>,
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

pub struct Scroll {
    pub total: usize,
    pub visible: usize,
    pub position: usize,
}

pub struct Menu<T> {
    pub options: HashMap<T, String>,
    pub hovered_index: Option<usize>,
    area: Rect,
}

pub struct MenuButton<T> {
    pub button: Button,
    pub menu: Menu<T>,
    pub is_open: bool,
    pub above: bool
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
            default: false,
            escape: false,
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

    pub fn default(mut self) -> Self {
        self.default = true;
        self
    }

    pub fn escape(mut self) -> Self {
        self.escape = true;
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
                if self.default && key_event.code == KeyCode::Enter {
                    return Some(());
                }
                if self.escape && key_event.code == KeyCode::Esc {
                    return Some(());
                }
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

        let mut style = Style::default().fg(self.color);
        if self.default {
            style = style.add_modifier(Modifier::BOLD);
        }

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
        if let Event::Key(key_event) = event {
            if key_event.code == KeyCode::Tab {
                let next_index = if self.active_index + 1 < self.options.len() {
                    self.active_index + 1
                } else {
                    0
                };
                self.set_active(next_index);
                return Some(next_index);
            }
        }

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

pub fn handle_scoped_event(focusables: &mut [&mut dyn Focusable], event: &Event) -> Option<InputEvent> {
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
                    return Some(InputEvent::FocusChanged);
                }

                // Focus the next widget
                focusables[index + 1].set_focused(true);
            } else {
                // If no focused widget, focus the first one
                if !focusables.is_empty() {
                    focusables[0].set_focused(true);
                }
            }
            return Some(InputEvent::FocusChanged);
        }

        // If esc is pressed, unfocus the current widget
        Event::Key(KeyEvent { code: KeyCode::Esc, .. }) => {
            if let Some(index) = focused_index {
                focusables[index].set_focused(false); // Unfocus the current widget
            }
            return Some(InputEvent::FocusChanged);
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
                InputEvent::Enter => {
                    // Enter is pressed, unfocus the current widget and focus the next one
                    focusables[index].set_focused(false);
                    if index + 1 < focusables.len() {
                        focusables[index + 1].set_focused(true);
                        return Some(InputEvent::FocusChanged);
                    }
                    return Some(InputEvent::FocusFinished);
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
            regex: None,
            area: Rect::default()
        }
    }

    pub fn masked(mut self) -> Self {
        self.masked = true;
        self
    }

    pub fn with_regex(mut self, regex: Regex) -> Self {
        self.regex = Some(regex);
        self
    }

    pub fn handle_input(&mut self, new_value: String) -> InputEvent {
        if let Some(regex) = &self.regex {
            if !regex.is_match(&new_value) {
                return InputEvent::ValidationFailed;
            }
        }
        self.value = new_value.clone().into();
        InputEvent::Input(new_value)
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

    fn handle_event(&mut self, event: &Event) -> Option<InputEvent> {
        if self.disabled {
            return None;
        }

        if let Event::Key(key_event) = event {
            match key_event.code {
                KeyCode::Char(c) => {
                    Some(self.handle_input(format!("{}{}", self.value.as_str(), c)))
                }
                KeyCode::Backspace => {
                    if self.value.is_empty() {
                        return None;
                    }
                    let new_value = self.value[..self.value.len() - 1].to_string();
                    Some(self.handle_input(new_value))
                }
                KeyCode::Enter => Some(InputEvent::Enter),
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

impl Scroll {
    pub fn new() -> Self {
        Self {
            total: 0,
            visible: 0,
            position: 0,
        }
    }

    pub fn handle_event(&mut self, event: &Event) -> Option<usize> {
        if self.visible >= self.total {
            return None;
        }

        match event {
            Event::Key(KeyEvent { code: KeyCode::Up, .. }) |
            Event::Mouse(MouseEvent { kind: MouseEventKind::ScrollUp, .. }) => {
                if self.position > 0 {
                    self.position -= 1;
                    return Some(self.position);
                }
            },
            Event::Key(KeyEvent { code: KeyCode::Down, .. }) |
            Event::Mouse(MouseEvent { kind: MouseEventKind::ScrollDown, .. }) => {
                if self.position < self.total - self.visible {
                    self.position += 1;
                    return Some(self.position);
                }
            },
            _ => {}
        }
        None
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        self.visible = area.height as usize;

        if self.total >= self.visible {
            let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
                .begin_symbol(Some(SCROLL_UP_SYMBOL))
                .end_symbol(Some(SCROLL_DOWN_SYMBOL))
                .thumb_symbol(SCROLL_THUMB_SYMBOL)
                .track_symbol(None)
                .style(Style::default().fg(Color::Yellow));

            let mut scrollbar_state = ScrollbarState::new(self.total - self.visible)
                .position(self.position);

            frame.render_stateful_widget(scrollbar, area, &mut scrollbar_state);
        } else {
            self.position = 0;
        }
    }
}

impl<T: Clone + Eq + std::hash::Hash> Menu<T> {
    pub fn new(options: HashMap<T, String>) -> Self {
        Self {
            options,
            hovered_index: None,
            area: Rect::default(),
        }
    }

    pub fn get_height(&self) -> u16 {
        self.options.len() as u16 + 2
    }

    pub fn handle_event(&mut self, event: &Event) -> Option<T> {
        match event {
            Event::Mouse(mouse_event) => {
                let inner = self.area.inner(Margin { vertical: 1, horizontal: 1 });
                if inner.contains(Position { x: mouse_event.column, y: mouse_event.row }) {
                    let option_height = inner.height / self.options.len() as u16;
                    let index = ((mouse_event.row - inner.y) / option_height) as usize;

                    if index < self.options.len() {
                        self.hovered_index = Some(index);
                        if mouse_event.kind == MouseEventKind::Down(MouseButton::Left) {
                            return self.options.keys().nth(index).cloned();
                        }
                    }
                }
            }
            Event::Key(key_event) => match key_event.code {
                KeyCode::Up => {
                    if let Some(index) = self.hovered_index {
                        self.hovered_index = Some(index.saturating_sub(1));
                    } else {
                        self.hovered_index = Some(self.options.len().saturating_sub(1));
                    }
                }
                KeyCode::Down => {
                    if let Some(index) = self.hovered_index {
                        if index < self.options.len() - 1 {
                            self.hovered_index = Some(index + 1);
                        }
                    } else {
                        self.hovered_index = Some(0);
                    }
                }
                KeyCode::Enter => {
                    if let Some(index) = self.hovered_index {
                        return self.options.keys().nth(index).cloned();
                    }
                }
                _ => {}
            },
            _ => {}
        }
        None
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        self.area = area; // Store the menu's area for mouse handling
        frame.render_widget(Clear, area);

        let menu_block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Yellow));
        frame.render_widget(menu_block, area);

        let options_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Length(1); self.options.len()])
            .margin(1)
            .split(area);

        for (i, option) in self.options.values().enumerate() {
            let style = if Some(i) == self.hovered_index {
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Yellow)
            };

            let option_paragraph = Paragraph::new(option.clone()).style(style);
            frame.render_widget(option_paragraph, options_layout[i]);
        }
    }
}

impl<T: Clone + Eq + std::hash::Hash> MenuButton<T> {
    pub fn new(label: &str, hotkey: Option<char>, options: HashMap<T, String>) -> Self {
        let button = Button::new(label, hotkey);
        let menu = Menu::new(options);

        Self {
            button,
            menu,
            is_open: false,
            above: false,
        }
    }

    pub fn keep_above(mut self) -> Self {
        self.above = true;
        self
    }

    pub fn handle_event(&mut self, event: &Event) -> Option<MenuEvent<T>> {
        if self.is_open {
            if let Some(selected) = self.menu.handle_event(event) {
                self.is_open = false;
                return Some(MenuEvent::Selected(selected));
            }

            if let Event::Mouse(mouse_event) = event {
                if mouse_event.kind == MouseEventKind::Down(MouseButton::Left) {
                    self.is_open = false;
                    return Some(MenuEvent::Closed);
                }
            } else if event == &Event::Key(KeyCode::Esc.into()) {
                self.is_open = false;
                return Some(MenuEvent::Closed);
            }
        } else if let Some(()) = self.button.handle_event(event) {
            self.is_open = true;
            return Some(MenuEvent::Opened);
        }
        None
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        self.button.render(frame, area);

        if self.is_open {
            let menu_height = self.menu.get_height();
            let menu_y = if self.above {
                area.y.saturating_sub(menu_height) + 1
            } else {
                area.y + area.height - 1
            };

            self.menu.render(frame, Rect { x: area.x, y: menu_y, width: area.width, height: menu_height });
        }
    }
}
