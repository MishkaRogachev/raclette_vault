use ratatui::{
    crossterm::event::Event,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Text, widgets::Paragraph,
    Frame
};

pub const SEED_PHRASE_HEIGHT: u16 = 20;

pub struct RevealSeedPhrase {
    phrase: String
}

impl RevealSeedPhrase {
    pub fn new(phrase: String) -> Self {
        RevealSeedPhrase { phrase }
    }
}

impl super::common::Widget for RevealSeedPhrase {
    fn handle_event(&mut self, event: Event) -> Option<Event> {
        Some(event)
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) {
        // Adjust the height dynamically to the terminal size
        let content_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(0), // Fill height
                Constraint::Length(SEED_PHRASE_HEIGHT),
                Constraint::Min(0), // Fill height
            ])
            .split(area);
    
        // Horizontal layout for two columns
        let columns_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)]) // 50% for each column
            .split(content_layout[1]);
    
        // Split the phrase into words
        let words: Vec<&str> = self.phrase.split(' ').collect();
        let half = words.len() / 2;
    
        let first_column_words = words[..half].to_vec();
        let second_column_words = words[half..].to_vec();

        // Define the vertical layout based on the number of words
        let first_column_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Length(1); first_column_words.len()])
            .split(columns_layout[0]);
    
        let second_column_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Length(1); second_column_words.len()])
            .split(columns_layout[1]);
    
        // Render the first column words
        for (i, word) in first_column_words.iter().enumerate() {
            frame.render_widget(render_centered_word(word), first_column_layout[i]);
        }
    
        // Render the second column words
        for (i, word) in second_column_words.iter().enumerate() {
            frame.render_widget(render_centered_word(word), second_column_layout[i]);
        }
    }
}

fn render_centered_word(word: &str) -> Paragraph<'_> {
    Paragraph::new(Text::raw(word))
        .alignment(ratatui::layout::Alignment::Center)
        .style(Style::default().fg(Color::Yellow))
}