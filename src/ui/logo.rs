use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::Paragraph,
    Frame,
};

pub fn big_logo(frame: &mut Frame) {
    let area = frame.area();

    let logo_width = 60;
    let horizontal_padding = (area.width.saturating_sub(logo_width)) / 2;

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(20),
                Constraint::Length(20),
                Constraint::Percentage(10),
            ]
            .as_ref(),
        )
        .horizontal_margin(horizontal_padding)
        .split(area);

    let logo = r#"
 ██▀███  ▄▄▄      ▄████▄  ██▓   ▓████▄▄▄█████▄▄▄█████▓█████ 
▓██ ▒ ██▒████▄   ▒██▀ ▀█ ▓██▒   ▓█   ▓  ██▒ ▓▓  ██▒ ▓▓█   ▀ 
▓██ ░▄█ ▒██  ▀█▄ ▒▓█    ▄▒██░   ▒███ ▒ ▓██░ ▒▒ ▓██░ ▒▒███   
▒██▀▀█▄ ░██▄▄▄▄██▒▓▓▄ ▄██▒██░   ▒▓█  ░ ▓██▓ ░░ ▓██▓ ░▒▓█  ▄ 
░██▓ ▒██▒▓█   ▓██▒ ▓███▀ ░██████░▒████▒▒██▒ ░  ▒██▒ ░░▒████▒
░ ▒▓ ░▒▓░▒▒   ▓▒█░ ░▒ ▒  ░ ▒░▓  ░░ ▒░ ░▒ ░░    ▒ ░░  ░░ ▒░ ░
  ░▒ ░ ▒░ ▒   ▒░ ░ ░  ▒    ░ ▒  ░░ ░  ░  ░       ░    ░    ░
   ░   ░      ░  ░           ░     ░           ░        ░   
         ██▒   █▓▄▄▄  ░   █    ██ ██▓ ▄▄▄█████▓             
        ▓██░   █▒████▄    ██  ▓██▓██▒ ▓  ██▒ ▓▒             
         ▓██  █▒▒██  ▀█▄ ▓██  ▒██▒██░ ▒ ▓██░ ▒░             
         ▒██ █░░██▄▄▄▄██▓▓█  ░██▒██░ ░ ▓██▓ ░               
          ▒▀█░  ▓█   ▓██▒▒█████▓░██████▒██▒ ░               
          ░ ▐░  ▒▒   ▓▒█░▒▓▒ ▒ ▒░ ▒░▓  ▒ ░                  
          ░ ░░   ▒   ▒▒ ░░▒░ ░  ░ ░ ▒  ░                    
            ░    ░   ▒   ░ ░        ░                       
"#;

    let paragraph = Paragraph::new(logo).style(Style::default().fg(Color::Yellow));
    frame.render_widget(paragraph, chunks[1]); // Centered vertically within the middle chunk
}
