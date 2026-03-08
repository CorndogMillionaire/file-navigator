use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};

use crate::app::App;

pub fn render(f: &mut Frame, app: &App, area: Rect) {
    let pal = &app.palette;
    let path = &app.current_dir;

    let mut spans = vec![Span::raw(" ")];

    let components: Vec<String> = path
        .components()
        .map(|c| c.as_os_str().to_string_lossy().to_uppercase())
        .collect();

    for (i, comp) in components.iter().enumerate() {
        if i > 0 {
            spans.push(Span::styled(" / ", Style::default().fg(pal.text_dim)));
        }
        let style = if i == components.len() - 1 {
            Style::default().fg(pal.text_hot)
        } else {
            Style::default().fg(pal.text_mid)
        };
        spans.push(Span::styled(comp.clone(), style));
    }

    // Blinking cursor
    if app.blink_on {
        spans.push(Span::styled(format!(" {}", app.symbols.blink_char), Style::default().fg(pal.text_hot)));
    } else {
        spans.push(Span::raw("  "));
    }

    let block = Block::default()
        .borders(Borders::BOTTOM)
        .border_style(Style::default().fg(pal.border_dim))
        .style(Style::default().bg(pal.bg));

    let paragraph = Paragraph::new(Line::from(spans)).block(block);
    f.render_widget(paragraph, area);
}
