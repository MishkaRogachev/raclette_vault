use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph},
    Frame
};

pub struct HProgress {
    pub min: u64,
    pub max: u64,
    pub value: u64,
    pub color: Color,
}

impl HProgress {
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
        let percentage = (self.value - self.min) as f64 / (self.max - self.min) as f64;

        let block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(self.color));
        let inner_area = block.inner(area);
        frame.render_widget(block, area);

        let filled_width = (area.width as f64 * percentage).round() as u16;

        if filled_width > 0 {
            let filled_area = Rect {
                x: inner_area.x,
                y: inner_area.y,
                width: filled_width,
                height: inner_area.height,
            };

            let filled_block = Block::default().style(Style::default().bg(self.color));
            frame.render_widget(filled_block, filled_area);
        }

        let paragraph = Paragraph::new(format!("{}/{}", self.value + 1, self.max))
            .style(Style::default().fg(self.color).add_modifier(Modifier::BOLD))
            .alignment(ratatui::layout::Alignment::Center);
        frame.render_widget(paragraph, inner_area);
    }
}
