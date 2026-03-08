use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, Paragraph};

use crate::app::{App, ThemePickerFocus};
use crate::palette::PALETTE_NAMES;
use crate::symbols::SYMBOL_SET_NAMES;

pub fn render(f: &mut Frame, app: &App, area: Rect) {
    let pal = &app.palette;
    let sym = &app.symbols;

    let popup_width = 64.min(area.width.saturating_sub(4));
    let list_height = PALETTE_NAMES.len().max(SYMBOL_SET_NAMES.len()) as u16;
    let popup_height = (list_height + 6).min(area.height.saturating_sub(4));

    let popup_area = centered_rect(popup_width, popup_height, area);
    f.render_widget(Clear, popup_area);

    let block = Block::default()
        .title(Line::from(vec![
            Span::styled(
                " T H E M E ",
                Style::default()
                    .fg(pal.text_mid)
                    .add_modifier(Modifier::BOLD),
            ),
        ]))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(pal.border_hot))
        .style(Style::default().bg(pal.surface));

    let inner = block.inner(popup_area);
    f.render_widget(block, popup_area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // column headers
            Constraint::Length(1), // separator
            Constraint::Min(1),   // tables
            Constraint::Length(1), // hint bar
        ])
        .split(inner);

    // Column headers
    let half_w = chunks[0].width / 2;
    let colors_focused = app.theme_picker_focus == ThemePickerFocus::Colors;
    let symbols_focused = app.theme_picker_focus == ThemePickerFocus::Symbols;

    let color_header_style = if colors_focused {
        Style::default()
            .fg(pal.text_hot)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(pal.text_dim)
    };
    let symbol_header_style = if symbols_focused {
        Style::default()
            .fg(pal.text_hot)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(pal.text_dim)
    };

    let header_line = Line::from(vec![
        Span::styled(
            format!(" {:<width$}", "COLOR PALETTE", width = half_w as usize - 1),
            color_header_style,
        ),
        Span::styled(
            format!("{:<width$}", "SYMBOL SET", width = half_w as usize),
            symbol_header_style,
        ),
    ]);
    f.render_widget(
        Paragraph::new(header_line).style(Style::default().bg(pal.surface)),
        chunks[0],
    );

    // Separator
    let sep_str: String = sym.horizontal_rule.repeat(chunks[1].width as usize);
    f.render_widget(
        Paragraph::new(Line::from(Span::styled(
            sep_str,
            Style::default().fg(pal.border_dim),
        )))
        .style(Style::default().bg(pal.surface)),
        chunks[1],
    );

    // Two-column table area
    let table_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[2]);

    // Color palette column
    render_column(
        f,
        app,
        table_chunks[0],
        PALETTE_NAMES,
        app.theme_picker_color_cursor,
        app.palette_index,
        colors_focused,
    );

    // Symbol set column
    render_column(
        f,
        app,
        table_chunks[1],
        SYMBOL_SET_NAMES,
        app.theme_picker_symbol_cursor,
        app.symbols_index,
        symbols_focused,
    );

    // Hint bar
    let sep = Span::styled(
        format!(" {} ", sym.separator),
        Style::default().fg(pal.border_mid),
    );
    let hint_line = Line::from(vec![
        Span::styled(" jk", Style::default().fg(pal.text_mid)),
        Span::styled(" nav", Style::default().fg(pal.text_dim)),
        sep.clone(),
        Span::styled("tab/hl", Style::default().fg(pal.text_mid)),
        Span::styled(" switch", Style::default().fg(pal.text_dim)),
        sep.clone(),
        Span::styled("enter", Style::default().fg(pal.text_mid)),
        Span::styled(" apply", Style::default().fg(pal.text_dim)),
        sep.clone(),
        Span::styled("esc", Style::default().fg(pal.text_mid)),
        Span::styled(" cancel", Style::default().fg(pal.text_dim)),
    ]);
    f.render_widget(
        Paragraph::new(hint_line).style(Style::default().bg(pal.surface)),
        chunks[3],
    );
}

fn render_column(
    f: &mut Frame,
    app: &App,
    area: Rect,
    names: &[&str],
    cursor: usize,
    active_index: usize,
    focused: bool,
) {
    let pal = &app.palette;
    let sym = &app.symbols;
    let height = area.height as usize;
    let mut lines: Vec<Line> = Vec::new();

    for (i, name) in names.iter().enumerate() {
        if i >= height {
            break;
        }
        let is_cursor = i == cursor;
        let is_active = i == active_index;

        let (bg, fg) = if is_cursor && focused {
            (pal.border_mid, pal.text_hot)
        } else if is_active {
            (pal.surface, pal.text_mid)
        } else {
            (pal.surface, pal.text_dim)
        };

        let indicator = if is_cursor && focused {
            sym.cursor
        } else if is_active {
            sym.separator
        } else {
            " "
        };

        let label = name.to_uppercase();
        lines.push(Line::from(vec![
            Span::styled(
                format!(" {} ", indicator),
                Style::default().fg(pal.text_hot).bg(bg),
            ),
            Span::styled(
                format!(
                    "{:<width$}",
                    label,
                    width = (area.width as usize).saturating_sub(4)
                ),
                Style::default().fg(fg).bg(bg),
            ),
        ]));
    }

    // Pad remaining
    for _ in lines.len()..height {
        lines.push(Line::from(Span::styled(
            " ".repeat(area.width as usize),
            Style::default().bg(pal.surface),
        )));
    }

    f.render_widget(
        Paragraph::new(lines).style(Style::default().bg(pal.surface)),
        area,
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
