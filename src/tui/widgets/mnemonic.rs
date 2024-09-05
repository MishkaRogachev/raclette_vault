use ratatui::{
    crossterm::event::Event,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Text, widgets::Paragraph,
    Frame
};

pub struct RevealWords {
    words: Vec<String>,
}

impl RevealWords {
    pub fn new(words: Vec<String>) -> Self {
        Self { words }
    }

    pub fn height(&self) -> u16 {
        (self.words.len() + 2) as u16
    }
}

impl super::common::Widget for RevealWords {
    fn handle_event(&mut self, event: Event) -> Option<Event> {
        Some(event)
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) {
        let content_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(0), // Fill height
                Constraint::Length(self.height()),
                Constraint::Min(0), // Fill height
            ])
            .split(area);

        let columns_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)]) // 50% for each column
            .split(content_layout[1]);

        let half = self.words.len() / 2;
        let first_column_words = self.words[..half].to_vec();
        let second_column_words = self.words[half..].to_vec();

        let word_height = std::cmp::max(1, area.height / (half as u16 - 1));

        for (i, word) in Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Length(word_height); first_column_words.len()])
            .split(columns_layout[0])
            .iter()
            .enumerate() {
            frame.render_widget(render_centered_word(&first_column_words[i]), *word);
        }

        for (i, word) in Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Length(word_height); second_column_words.len()])
            .split(columns_layout[1])
            .iter()
            .enumerate() {
            frame.render_widget(render_centered_word(&second_column_words[i]), *word);
        }
    }
}

fn render_centered_word(word: &str) -> Paragraph<'_> {
    Paragraph::new(Text::raw(word))
        .alignment(ratatui::layout::Alignment::Center)
        .style(Style::default().fg(Color::Yellow))
}