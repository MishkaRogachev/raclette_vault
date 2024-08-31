use ratatui::{
    layout::Alignment,
    style::{Color, Style},
    widgets::{Block, Paragraph},
    Frame,
};

const BIG_LOGO_ASCII: &str = r#"
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

pub const BIG_LOGO_WIDTH: u16 = 60;
pub const BIG_LOGO_HEIGHT: u16 = 20;

pub fn big_logo(area: ratatui::layout::Rect, frame: &mut Frame) {
    let paragraph = Paragraph::new(BIG_LOGO_ASCII)
        .style(Style::default().fg(Color::Yellow))
        .alignment(Alignment::Center)
        .block(Block::default());

    frame.render_widget(paragraph, area);
}
