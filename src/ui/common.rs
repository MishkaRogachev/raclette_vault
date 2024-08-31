use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph},
};

pub fn button<'a>(hotkey: &'a str, label: &'a str) -> Paragraph<'a> {
    Paragraph::new(Text::from(Line::from(vec![
        Span::styled(hotkey, Style::default().fg(Color::Yellow).add_modifier(Modifier::UNDERLINED)),
        Span::styled(label, Style::default().fg(Color::Yellow)),
    ])))
    .alignment(ratatui::layout::Alignment::Center)
    .block(Block::default().borders(Borders::ALL).style(Style::default().fg(Color::Yellow)))
}
