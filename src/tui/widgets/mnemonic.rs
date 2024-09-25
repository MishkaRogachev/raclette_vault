use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Text, widgets::Paragraph,
    Frame
};
use zeroize::Zeroizing;

const MASKED_PLACEHOLDER: &str = "******";
pub const MNEMONIC_HEIGHT: u16 = 30;

pub struct MnemonicWords {
    pub words: Vec<Zeroizing<String>>,
    pub masked: bool,
    pub color: Color,
}

impl MnemonicWords {
    pub fn new(words: Vec<Zeroizing<String>>) -> Self {
        Self { words, masked: true, color: Color::Yellow }
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        let word_count = self.words.len();
        if word_count == 0 {
            return;
        }

        let column_count = if word_count <= 12 { 2 } else { 4 };
        let words_per_column = (word_count + column_count - 1) / column_count;
        let word_height = std::cmp::max(1, area.height / (words_per_column as u16));

        let content_layout = Layout::default()
            .direction(Direction::Vertical)
            .horizontal_margin(area.width / column_count as u16 / 2)
            .constraints([
                Constraint::Fill(0), // Fill height
                Constraint::Length(word_height * words_per_column as u16),
                Constraint::Fill(0), // Fill height
            ])
            .split(area);

        let columns_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(100 / column_count as u16); column_count])
            .split(content_layout[1]);

        for (col_idx, column_area) in columns_layout.iter().enumerate() {
            let start_idx = col_idx * words_per_column;
            let end_idx = std::cmp::min(start_idx + words_per_column, word_count);
            let words_in_column = &self.words[start_idx..end_idx];

            let word_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![Constraint::Length(word_height); words_in_column.len()])
                .split(*column_area);

            for (i, word_area) in word_layout.iter().enumerate() {
                let word = if self.masked {
                    MASKED_PLACEHOLDER
                } else {
                    &words_in_column[i]
                };
                frame.render_widget(render_centered_word(word, start_idx + i, self.color), *word_area);
            }
        }
    }
}

fn render_centered_word(word: &str, i: usize, color: Color) -> Paragraph<'_> {
    let word = format!("{}) {}", i + 1, word);
    Paragraph::new(Text::raw(word))
        .alignment(ratatui::layout::Alignment::Left)
        .style(Style::default().fg(color))
}
