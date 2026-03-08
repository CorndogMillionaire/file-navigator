use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};

use crate::app::App;

pub fn render(f: &mut Frame, app: &App, area: Rect) {
    let pal = &app.palette;
    let item_count = app.entries.len();

    let sym = &app.symbols;
    let left = vec![
        Span::styled(
            " REM NAVIGATOR",
            Style::default().fg(pal.text_hot).add_modifier(Modifier::BOLD),
        ),
        Span::styled(format!(" {} ", sym.separator), Style::default().fg(pal.text_dim)),
        Span::styled("FILE SYSTEM", Style::default().fg(pal.text_mid)),
    ];

    let palette_label = app.palette_name().to_uppercase();
    let right_text = format!(
        "ITEMS:{} {} THEME:{} {} SYS:NOMINAL ",
        item_count, sym.separator, palette_label, sym.separator
    );
    let right = vec![Span::styled(
        right_text,
        Style::default().fg(pal.text_mid),
    )];

    // Calculate padding
    let left_len: usize = left.iter().map(|s| s.width()).sum();
    let right_len: usize = right.iter().map(|s| s.width()).sum();
    let pad = if area.width as usize > left_len + right_len {
        area.width as usize - left_len - right_len
    } else {
        1
    };

    let mut spans = left;
    spans.push(Span::raw(" ".repeat(pad)));
    spans.extend(right);

    let block = Block::default()
        .borders(Borders::BOTTOM)
        .border_style(Style::default().fg(pal.border_mid))
        .style(Style::default().bg(pal.surface));

    let paragraph = Paragraph::new(Line::from(spans)).block(block);
    f.render_widget(paragraph, area);
}
