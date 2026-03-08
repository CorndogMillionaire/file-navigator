use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState};

use crate::app::{App, Mode};
use crate::nav;
use crate::palette::Palette;

pub fn render(f: &mut Frame, app: &App, area: Rect) {
    let pal = &app.palette;
    let sym = &app.symbols;
    let width = area.width as usize;
    let height = area.height as usize;

    let (entries, indices) = app.display_entries();

    if indices.is_empty() {
        let msg = Paragraph::new(Line::from(Span::styled(
            " NO ENTRIES",
            Style::default().fg(pal.text_dim),
        )))
        .style(Style::default().bg(pal.bg));
        f.render_widget(msg, area);
        return;
    }

    let show_size = width >= 90;
    let show_type = width >= 80;
    let in_jump_mode = app.mode == Mode::JumpKey;
    let in_fuzzy_mode = app.mode == Mode::FuzzySearch;

    let total = indices.len();
    let start = app.scroll_offset;
    let end = (start + height).min(total);

    let mut lines: Vec<Line> = Vec::new();

    for view_idx in start..end {
        let entry_idx = indices[view_idx];
        let entry = &entries[entry_idx];
        let is_cursor = view_idx == app.cursor;

        // Depth-based color adjustment for fuzzy results
        let depth_color = depth_text_color(pal, entry.depth);

        let (row_bg, text_color) = if is_cursor {
            (pal.surface, pal.text_hot)
        } else if in_fuzzy_mode && !app.fuzzy_query.is_empty() {
            (pal.bg, depth_color)
        } else if in_jump_mode {
            (pal.bg, pal.text_dim)
        } else {
            (pal.bg, pal.text_dim)
        };

        let mut spans: Vec<Span> = Vec::new();

        // Selection indicator
        if is_cursor {
            spans.push(Span::styled(
                format!("{} ", sym.cursor),
                Style::default().fg(pal.text_hot).bg(row_bg),
            ));
        } else {
            spans.push(Span::styled("  ", Style::default().bg(row_bg)));
        }

        // Jump key column (5 chars)
        if in_jump_mode {
            if let Some((k1, k2)) = app.jump_key_for_entry(entry_idx) {
                let dimmed = app.pending_jump_key.is_some()
                    && app.pending_jump_key != Some(k1);
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
        }

        // Depth indicator for fuzzy results from other directories
        if in_fuzzy_mode && !app.fuzzy_query.is_empty() && entry.depth != 0 {
            let depth_badge = if entry.depth < 0 {
                format!("{} ", sym.depth_up)
            } else {
                format!("{} ", sym.depth_down)
            };
            spans.push(Span::styled(
                depth_badge,
                Style::default().fg(depth_color).bg(row_bg),
            ));
        } else {
            // Sigil
            let sigil = if entry.is_dir {
                format!("{} ", sym.dir_sigil)
            } else {
                format!("{} ", sym.file_sigil)
            };
            spans.push(Span::styled(
                sigil,
                Style::default().fg(text_color).bg(row_bg),
            ));
        }

        // Name - with fuzzy highlighting if active
        let name_style = if is_cursor {
            Style::default().fg(pal.text_hot).bg(row_bg)
        } else if in_fuzzy_mode && !app.fuzzy_query.is_empty() {
            Style::default().fg(depth_color).bg(row_bg)
        } else {
            Style::default()
                .fg(if entry.is_dir {
                    pal.text_mid
                } else {
                    text_color
                })
                .bg(row_bg)
        };

        let display_name = if entry.is_dir && !entry.name.ends_with('/') {
            format!("{}/", entry.name)
        } else if entry.is_dir {
            entry.name.clone()
        } else {
            entry.name.clone()
        };

        // Calculate available name width
        let prefix_width = 2 // selection indicator
            + if in_jump_mode { 5 } else { 0 }
            + 2; // sigil/depth
        let suffix_width = if show_type { 6 } else { 0 } + if show_size { 9 } else { 0 };
        let name_width = width.saturating_sub(prefix_width + suffix_width + 1);

        if in_fuzzy_mode && !app.fuzzy_query.is_empty() {
            // Use basename for match indices, but display full relative name
            let basename = entry
                .path
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_else(|| entry.name.clone());
            let match_indices = app.fuzzy_match_indices(&basename);
            let truncated = truncate_str(&display_name, name_width);

            // Figure out where the basename starts in the display name
            let basename_offset = if display_name.len() > basename.len() {
                display_name.len() - basename.len() - if entry.is_dir { 1 } else { 0 }
            } else {
                0
            };

            for (i, ch) in truncated.chars().enumerate() {
                let adjusted = if i >= basename_offset {
                    i - basename_offset
                } else {
                    usize::MAX
                };
                let style = if match_indices.contains(&adjusted) {
                    Style::default()
                        .fg(pal.text_hot)
                        .bg(row_bg)
                        .add_modifier(Modifier::BOLD)
                } else {
                    name_style
                };
                spans.push(Span::styled(ch.to_string(), style));
            }
            let char_count = truncated.chars().count();
            if char_count < name_width {
                spans.push(Span::styled(
                    " ".repeat(name_width - char_count),
                    Style::default().bg(row_bg),
                ));
            }
        } else {
            let truncated = truncate_str(&display_name, name_width);
            let pad = name_width.saturating_sub(truncated.chars().count());
            spans.push(Span::styled(truncated, name_style));
            if pad > 0 {
                spans.push(Span::styled(
                    " ".repeat(pad),
                    Style::default().bg(row_bg),
                ));
            }
        }

        // Type badge
        if show_type {
            let badge = nav::type_badge_str(entry);
            spans.push(Span::styled(
                format!("{:>5} ", badge),
                Style::default().fg(pal.text_dim).bg(row_bg),
            ));
        }

        // Size
        if show_size {
            let size_str = if entry.is_dir {
                sym.em_dash.to_string()
            } else {
                entry
                    .size
                    .map(nav::format_size)
                    .unwrap_or_else(|| sym.em_dash.to_string())
            };
            spans.push(Span::styled(
                format!("{:>8} ", size_str),
                Style::default().fg(pal.text_dim).bg(row_bg),
            ));
        }

        lines.push(Line::from(spans));
    }

    // Pad remaining lines
    for _ in lines.len()..height {
        lines.push(Line::from(Span::styled(
            " ".repeat(width),
            Style::default().bg(pal.bg),
        )));
    }

    let paragraph = Paragraph::new(lines).style(Style::default().bg(pal.bg));
    f.render_widget(paragraph, area);

    // Scrollbar
    if total > height {
        let mut scrollbar_state = ScrollbarState::new(total).position(app.scroll_offset);
        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .thumb_symbol(sym.scrollbar_thumb)
            .track_symbol(Some(sym.scrollbar_track))
            .thumb_style(Style::default().fg(pal.border_mid))
            .track_style(Style::default().fg(pal.border_dim));
        f.render_stateful_widget(scrollbar, area, &mut scrollbar_state);
    }
}

/// Map depth to text color: depth 0 = brightest, further away = dimmer
fn depth_text_color(pal: &Palette, depth: i32) -> ratatui::style::Color {
    match depth.abs() {
        0 => pal.text_mid,
        1 => pal.text_dim,
        _ => pal.border_mid,
    }
}

fn truncate_str(s: &str, max_width: usize) -> String {
    truncate_str_with(s, max_width, "\u{2026}")
}

fn truncate_str_with(s: &str, max_width: usize, ellipsis: &str) -> String {
    let chars: Vec<char> = s.chars().collect();
    if chars.len() <= max_width {
        s.to_string()
    } else if max_width > ellipsis.len() {
        chars[..max_width - ellipsis.len()].iter().collect::<String>() + ellipsis
    } else {
        ellipsis.to_string()
    }
}
