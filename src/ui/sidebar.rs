use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};

use crate::app::App;
use crate::nav;
use crate::palette;

pub fn render(f: &mut Frame, app: &App, area: Rect) {
    let pal = &app.palette;
    let mut lines: Vec<Line> = Vec::new();

    let block = Block::default()
        .borders(Borders::LEFT)
        .border_style(Style::default().fg(pal.border_dim))
        .style(Style::default().bg(pal.bg));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let label_style = Style::default().fg(pal.text_dim);
    let value_style = Style::default().fg(pal.text_hot);
    let key_style = Style::default().fg(pal.text_dim);

    // ASCII art corpo logo
    let logo_lines = palette::corpo_logo(app.palette_name());
    for line in logo_lines {
        lines.push(Line::from(Span::styled(
            *line,
            Style::default().fg(pal.border_mid),
        )));
    }
    lines.push(Line::from(""));

    // SELECTION section
    lines.push(Line::from(Span::styled(
        " S E L E C T I O N",
        label_style.add_modifier(Modifier::BOLD),
    )));

    if let Some(entry) = app.current_entry() {
        let kv = |key: &str, val: String| -> Line {
            Line::from(vec![
                Span::styled(format!(" {:<7}", key), key_style),
                Span::styled(val, value_style),
            ])
        };

        let display_name = if entry.name.len() > (inner.width as usize).saturating_sub(9) {
            let max = (inner.width as usize).saturating_sub(10);
            let truncated: String = entry.name.chars().take(max).collect();
            format!("{}{}", truncated, app.symbols.ellipsis)
        } else {
            entry.name.clone()
        };

        lines.push(kv("NAME", display_name));
        lines.push(kv(
            "TYPE",
            if entry.is_dir {
                "DIR".to_string()
            } else {
                nav::type_badge_str(entry)
            },
        ));
        lines.push(kv(
            "SIZE",
            entry
                .size
                .map(nav::format_size)
                .unwrap_or_else(|| app.symbols.em_dash.to_string()),
        ));
        if let Some(ref perms) = entry.permissions {
            lines.push(kv("PERMS", perms.clone()));
        }
        if let Some(modified) = entry.modified {
            lines.push(kv("MOD", nav::format_modified(modified)));
        }
    } else {
        lines.push(Line::from(Span::styled(" (NONE)", label_style)));
    }

    lines.push(Line::from(""));

    // BOOKMARKS section
    lines.push(Line::from(Span::styled(
        " B O O K M A R K S",
        label_style.add_modifier(Modifier::BOLD),
    )));

    if app.marks.is_empty() {
        lines.push(Line::from(Span::styled(
            " (NONE) B to add",
            label_style,
        )));
    } else {
        let mut sorted_marks: Vec<_> = app.marks.iter().collect();
        sorted_marks.sort_by_key(|(k, _)| *k);
        let max_marks = (inner.height as usize).saturating_sub(lines.len() + 2);
        for (key, path) in sorted_marks.iter().take(max_marks) {
            let display = path
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_else(|| path.to_string_lossy().to_string());
            lines.push(Line::from(vec![
                Span::styled(
                    format!(" [{}] ", key),
                    Style::default()
                        .fg(pal.text_hot)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(display, key_style),
            ]));
        }
        lines.push(Line::from(Span::styled(
            " b:open  B:add",
            Style::default().fg(pal.border_mid),
        )));
    }

    let paragraph = Paragraph::new(lines).style(Style::default().bg(pal.bg));
    f.render_widget(paragraph, inner);
}
