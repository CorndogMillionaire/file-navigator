use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};

use crate::app::FsEntry;
use crate::palette::Palette;
use crate::symbols::SymbolSet;

/// Render a dim side segment (parent or child preview).
/// `highlight` is the index of the entry to highlight (e.g., current dir in parent).
/// `jump_keys` maps entry index -> (char, char) label for jump mode.
/// `pending_first` is the first key already pressed in jump mode (to dim non-matching).
pub fn render(
    f: &mut Frame,
    entries: &[FsEntry],
    highlight: Option<usize>,
    pal: &Palette,
    sym: &SymbolSet,
    area: Rect,
    show_border_left: bool,
    jump_keys: &[(usize, (char, char))],
    pending_first: Option<char>,
) {
    let block = if show_border_left {
        Block::default()
            .borders(Borders::LEFT)
            .border_style(Style::default().fg(pal.border_dim))
            .style(Style::default().bg(pal.bg))
    } else {
        Block::default()
            .borders(Borders::RIGHT)
            .border_style(Style::default().fg(pal.border_dim))
            .style(Style::default().bg(pal.bg))
    };

    let inner = block.inner(area);
    f.render_widget(block, area);

    let inner_width = inner.width as usize;
    let inner_height = inner.height as usize;

    if entries.is_empty() {
        let lines = vec![Line::from(Span::styled(
            " (EMPTY)",
            Style::default().fg(pal.border_mid).bg(pal.bg),
        ))];
        let paragraph = Paragraph::new(lines).style(Style::default().bg(pal.bg));
        f.render_widget(paragraph, inner);
        return;
    }

    // Scroll to keep highlight visible
    let scroll_offset = if let Some(hi) = highlight {
        if hi >= inner_height {
            hi.saturating_sub(inner_height / 2)
        } else {
            0
        }
    } else {
        0
    };

    let in_jump_mode = !jump_keys.is_empty();

    let start = scroll_offset;
    let end = (start + inner_height).min(entries.len());

    let mut lines: Vec<Line> = Vec::new();

    for i in start..end {
        let entry = &entries[i];
        let is_highlighted = Some(i) == highlight;

        let (row_bg, text_color) = if is_highlighted {
            (pal.surface, pal.text_mid)
        } else {
            (pal.bg, pal.text_dim)
        };

        let mut spans: Vec<Span> = Vec::new();

        // Jump key column (if in jump mode)
        if in_jump_mode {
            if let Some((_, (k1, k2))) = jump_keys.iter().find(|(idx, _)| *idx == i) {
                let dimmed = pending_first.is_some() && pending_first != Some(*k1);
                if dimmed {
                    spans.push(Span::styled("     ", Style::default().bg(row_bg)));
                } else {
                    spans.push(Span::styled(
                        format!("[{}{}] ", k1, k2),
                        Style::default()
                            .fg(pal.text_hot)
                            .bg(pal.border_mid)
                            .add_modifier(Modifier::BOLD),
                    ));
                }
            } else {
                spans.push(Span::styled("     ", Style::default().bg(row_bg)));
            }
        } else {
            // Cursor indicator for highlighted entry
            if is_highlighted {
                spans.push(Span::styled(
                    format!("{}", sym.cursor),
                    Style::default().fg(pal.text_mid).bg(row_bg),
                ));
            } else {
                spans.push(Span::styled(" ", Style::default().bg(row_bg)));
            }
        }

        // Sigil
        let sigil = if entry.is_dir { sym.dir_sigil } else { sym.file_sigil };
        spans.push(Span::styled(
            format!("{} ", sigil),
            Style::default().fg(text_color).bg(row_bg),
        ));

        // Name — truncated to fit
        let prefix_len = if in_jump_mode { 5 + 2 } else { 1 + 2 }; // jump/cursor + sigil+space
        let name_width = inner_width.saturating_sub(prefix_len);
        let display_name = if entry.is_dir && !entry.name.ends_with('/') {
            format!("{}/", entry.name)
        } else {
            entry.name.clone()
        };
        let truncated = truncate_str(&display_name, name_width);
        let pad = name_width.saturating_sub(truncated.chars().count());
        spans.push(Span::styled(truncated, Style::default().fg(text_color).bg(row_bg)));
        if pad > 0 {
            spans.push(Span::styled(
                " ".repeat(pad),
                Style::default().bg(row_bg),
            ));
        }

        lines.push(Line::from(spans));
    }

    // Pad remaining lines
    for _ in lines.len()..inner_height {
        lines.push(Line::from(Span::styled(
            " ".repeat(inner_width),
            Style::default().bg(pal.bg),
        )));
    }

    let paragraph = Paragraph::new(lines).style(Style::default().bg(pal.bg));
    f.render_widget(paragraph, inner);
}

fn truncate_str(s: &str, max_width: usize) -> String {
    let chars: Vec<char> = s.chars().collect();
    if chars.len() <= max_width {
        s.to_string()
    } else if max_width > 1 {
        chars[..max_width - 1].iter().collect::<String>() + "\u{2026}"
    } else {
        "\u{2026}".to_string()
    }
}
