use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

use crate::app::App;

pub fn render(f: &mut Frame, app: &App, area: Rect) {
    let pal = &app.palette;

    if area.height == 0 {
        return;
    }
    let row_area = Rect {
        x: area.x,
        y: area.y + area.height - 1,
        width: area.width,
        height: 1,
    };

    let match_count = app.fuzzy_filtered.len();
    let pool_total = app.fuzzy_pool.len();
    let right_text = format!("[{}/{} matches]", match_count, pool_total);
    let right_len = right_text.len();

    let cursor_char = if app.blink_on { app.symbols.blink_char } else { " " };

    let mut spans = vec![
        Span::styled(
            " [/] ",
            Style::default().fg(pal.text_hot).bg(pal.border_hot),
        ),
        Span::styled(
            &app.fuzzy_query,
            Style::default().fg(pal.text_hot).bg(pal.bg),
        ),
        Span::styled(cursor_char, Style::default().fg(pal.text_hot).bg(pal.bg)),
    ];

    let used = 5 + app.fuzzy_query.len() + 1 + right_len;
    let pad = (row_area.width as usize).saturating_sub(used);
    spans.push(Span::styled(
        " ".repeat(pad),
        Style::default().bg(pal.bg),
    ));
    spans.push(Span::styled(
        right_text,
        Style::default().fg(pal.text_dim).bg(pal.bg),
    ));

    let paragraph = Paragraph::new(Line::from(spans)).style(Style::default().bg(pal.bg));
    f.render_widget(paragraph, row_area);
}
