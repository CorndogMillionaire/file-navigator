use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, Paragraph};

use crate::app::App;

pub fn render(f: &mut Frame, app: &App, area: Rect) {
    let pal = &app.palette;

    // Center the popup
    let popup_width = 60.min(area.width.saturating_sub(4));
    let popup_height = (app.bookmark_filtered.len() as u16 + 5).min(area.height.saturating_sub(4));

    let popup_area = centered_rect(popup_width, popup_height, area);

    // Clear the area behind the popup
    f.render_widget(Clear, popup_area);

    let block = Block::default()
        .title(Line::from(vec![
            Span::styled(
                " \u{2691} ",
                Style::default()
                    .fg(pal.text_hot)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "B O O K M A R K S",
                Style::default()
                    .fg(pal.text_mid)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" "),
        ]))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(pal.border_hot))
        .style(Style::default().bg(pal.surface));

    let inner = block.inner(popup_area);
    f.render_widget(block, popup_area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // search row
            Constraint::Length(1), // separator
            Constraint::Min(1),   // bookmark list
            Constraint::Length(1), // hint bar
        ])
        .split(inner);

    // Search row
    let cursor_char = if app.blink_on { "\u{258b}" } else { " " };
    let search_line = Line::from(vec![
        Span::styled(
            " [/] ",
            Style::default().fg(pal.text_hot).bg(pal.border_mid),
        ),
        Span::styled(
            &app.bookmark_query,
            Style::default().fg(pal.text_hot).bg(pal.surface),
        ),
        Span::styled(
            cursor_char,
            Style::default().fg(pal.text_hot).bg(pal.surface),
        ),
    ]);
    f.render_widget(
        Paragraph::new(search_line).style(Style::default().bg(pal.surface)),
        chunks[0],
    );

    // Separator
    let sep_str: String = "\u{2500}".repeat(chunks[1].width as usize);
    f.render_widget(
        Paragraph::new(Line::from(Span::styled(
            sep_str,
            Style::default().fg(pal.border_dim),
        )))
        .style(Style::default().bg(pal.surface)),
        chunks[1],
    );

    // Bookmark list
    let list_height = chunks[2].height as usize;
    let mut lines: Vec<Line> = Vec::new();

    if app.bookmark_filtered.is_empty() {
        lines.push(Line::from(Span::styled(
            "  NO MATCHES",
            Style::default().fg(pal.text_dim),
        )));
    } else {
        for (i, key) in app.bookmark_filtered.iter().enumerate() {
            if i >= list_height {
                break;
            }
            let is_selected = i == app.bookmark_cursor;
            let path = app.marks.get(key).map(|p| p.to_string_lossy().to_string()).unwrap_or_default();
            let dir_name = app
                .marks
                .get(key)
                .and_then(|p| p.file_name())
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_else(|| path.clone());

            let (bg, fg_key, fg_path) = if is_selected {
                (pal.border_mid, pal.text_hot, pal.text_hot)
            } else {
                (pal.surface, pal.text_mid, pal.text_dim)
            };

            let indicator = if is_selected { "\u{25b6}" } else { " " };

            lines.push(Line::from(vec![
                Span::styled(
                    format!(" {} ", indicator),
                    Style::default().fg(pal.text_hot).bg(bg),
                ),
                Span::styled(
                    format!("[{}] ", key),
                    Style::default()
                        .fg(fg_key)
                        .bg(bg)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    truncate_path(&dir_name, 16),
                    Style::default().fg(fg_key).bg(bg),
                ),
                Span::styled(
                    format!(
                        "  {}",
                        truncate_path_left(&path, (chunks[2].width as usize).saturating_sub(26))
                    ),
                    Style::default().fg(fg_path).bg(bg),
                ),
            ]));
        }
    }

    // Pad remaining
    for _ in lines.len()..list_height {
        lines.push(Line::from(Span::styled(
            " ".repeat(chunks[2].width as usize),
            Style::default().bg(pal.surface),
        )));
    }

    f.render_widget(
        Paragraph::new(lines).style(Style::default().bg(pal.surface)),
        chunks[2],
    );

    // Hint bar
    let hint_line = Line::from(vec![
        Span::styled(
            " enter",
            Style::default().fg(pal.text_mid),
        ),
        Span::styled(" go", Style::default().fg(pal.text_dim)),
        Span::styled(
            " \u{00b7} ",
            Style::default().fg(pal.border_mid),
        ),
        Span::styled(
            "\u{2191}\u{2193}",
            Style::default().fg(pal.text_mid),
        ),
        Span::styled(" nav", Style::default().fg(pal.text_dim)),
        Span::styled(
            " \u{00b7} ",
            Style::default().fg(pal.border_mid),
        ),
        Span::styled(
            "^d",
            Style::default().fg(pal.text_mid),
        ),
        Span::styled(" del", Style::default().fg(pal.text_dim)),
        Span::styled(
            " \u{00b7} ",
            Style::default().fg(pal.border_mid),
        ),
        Span::styled("esc", Style::default().fg(pal.text_mid)),
        Span::styled(" close", Style::default().fg(pal.text_dim)),
    ]);
    f.render_widget(
        Paragraph::new(hint_line).style(Style::default().bg(pal.surface)),
        chunks[3],
    );
}

fn centered_rect(width: u16, height: u16, area: Rect) -> Rect {
    let x = area.x + (area.width.saturating_sub(width)) / 2;
    let y = area.y + (area.height.saturating_sub(height)) / 2;
    Rect {
        x,
        y,
        width: width.min(area.width),
        height: height.min(area.height),
    }
}

fn truncate_path(s: &str, max: usize) -> String {
    if s.len() <= max {
        format!("{:<width$}", s, width = max)
    } else if max > 1 {
        format!("{}\u{2026}", &s[..max - 1])
    } else {
        "\u{2026}".to_string()
    }
}

fn truncate_path_left(s: &str, max: usize) -> String {
    if s.len() <= max {
        s.to_string()
    } else if max > 1 {
        format!("\u{2026}{}", &s[s.len() - max + 1..])
    } else {
        "\u{2026}".to_string()
    }
}
