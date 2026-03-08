use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};

use crate::app::{App, Mode};

pub fn render(f: &mut Frame, app: &App, area: Rect) {
    let pal = &app.palette;

    let block = Block::default()
        .borders(Borders::TOP)
        .border_style(Style::default().fg(pal.border_dim))
        .style(Style::default().bg(pal.surface));

    // Error state overrides footer
    if let Some((ref msg, _)) = app.error {
        let line = Line::from(vec![
            Span::styled(
                format!(" \u{26a0} {}", msg),
                Style::default().fg(pal.warn),
            ),
            Span::raw("  "),
            Span::styled("[ANY KEY TO DISMISS]", Style::default().fg(pal.warn)),
        ]);
        let paragraph = Paragraph::new(line).block(block);
        f.render_widget(paragraph, area);
        return;
    }

    let sep = Span::styled(" \u{00b7} ", Style::default().fg(pal.border_mid));
    let key_style = Style::default().fg(pal.text_mid);
    let desc_style = Style::default().fg(pal.text_dim);

    let spans = match app.mode {
        Mode::Normal => {
            vec![
                Span::raw(" "),
                Span::styled("hjkl", key_style),
                Span::styled(" move", desc_style),
                sep.clone(),
                Span::styled("space", key_style),
                Span::styled(" jump", desc_style),
                sep.clone(),
                Span::styled("/", key_style),
                Span::styled(" fuzzy", desc_style),
                sep.clone(),
                Span::styled("s", key_style),
                Span::styled(" select dir", desc_style),
                sep.clone(),
                Span::styled("b", key_style),
                Span::styled(" bookmarks", desc_style),
                sep.clone(),
                Span::styled("B", key_style),
                Span::styled(" +mark", desc_style),
                sep.clone(),
                Span::styled("t", key_style),
                Span::styled(" theme", desc_style),
                sep.clone(),
                Span::styled("q", key_style),
                Span::styled(" quit", desc_style),
            ]
        }
        Mode::FuzzySearch => {
            vec![
                Span::raw(" "),
                Span::styled("type", key_style),
                Span::styled(" to filter", desc_style),
                sep.clone(),
                Span::styled("\u{2191}\u{2193}", key_style),
                Span::styled(" navigate", desc_style),
                sep.clone(),
                Span::styled("enter", key_style),
                Span::styled(" select", desc_style),
                sep.clone(),
                Span::styled("esc", key_style),
                Span::styled(" cancel", desc_style),
            ]
        }
        Mode::JumpKey => {
            vec![
                Span::raw(" "),
                Span::styled("a-z", key_style),
                Span::styled(" jump to entry", desc_style),
                sep.clone(),
                Span::styled("esc", key_style),
                Span::styled(" cancel", desc_style),
            ]
        }
        Mode::Bookmark => {
            vec![
                Span::raw(" "),
                Span::styled("\u{2691} BOOKMARK MODE", key_style),
            ]
        }
    };

    let paragraph = Paragraph::new(Line::from(spans)).block(block);
    f.render_widget(paragraph, area);
}
