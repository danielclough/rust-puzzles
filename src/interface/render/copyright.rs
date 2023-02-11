use tui::{
    layout::Alignment,
    style::{Color,Style},
    widgets::{Block, BorderType, Borders, Paragraph},
};

pub fn copyright_component() -> Paragraph<'static> {
    Paragraph::new("© 2023 Rust Quizzes - All rights reserved.")
        .style(Style::default().fg(Color::LightCyan))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .title("Copyright")
                .border_type(BorderType::Plain),
        )
}
