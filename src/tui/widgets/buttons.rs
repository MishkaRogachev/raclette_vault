use ratatui::{
    crossterm::event::{Event, KeyCode, MouseButton, MouseEventKind},
    layout::{Constraint, Direction, Layout, Position, Rect},
    style::{Color, Modifier, Style, Stylize},
    symbols,
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame
};

const BUTTONS_HEIGHT: u16 = 3;

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

        let style = Style::default().fg(self.color).add_modifier(Modifier::BOLD);
        let block = Block::default().borders(Borders::ALL).border_style(style);

        let block = if self.disabled {
            block.dim()
        } else { 
            if self.active { block } else { block } // FIXME: active look
        };

        let (block, style) = if !self.disabled && self.is_hovered {
            ( block.border_set(symbols::border::DOUBLE), style.bg(self.color).fg(Color::Black))
        } else {
            (block, style)
        };

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

    pub fn handle_event(&mut self, event: &Event) -> Option<bool> {
        let button = if self.state { &mut self.second } else { &mut self.first };
        if let Some(()) = button.handle_event(event) {
            self.state = !self.state;
            Some(self.state)
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
            frame.render_widget(background_block, menu_area); // Yellow background for the popup

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
