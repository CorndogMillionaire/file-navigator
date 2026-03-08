mod bookmark;
mod breadcrumb;
mod footer;
mod fuzzy;
mod header;
mod jumpkey;
mod list;
mod sidebar;
mod theme_picker;

use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::Style;
use ratatui::widgets::Block;

use crate::app::{App, Mode};

pub fn render(f: &mut Frame, app: &mut App) {
    let area = f.area();
    let pal = &app.palette;

    // Fill background
    let bg_block = Block::default().style(Style::default().bg(pal.bg));
    f.render_widget(bg_block, area);

    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2), // header + border
            Constraint::Length(2), // breadcrumb + border
            Constraint::Min(3),   // body
            Constraint::Length(2), // footer + border
        ])
        .split(area);

    header::render(f, app, main_chunks[0]);
    breadcrumb::render(f, app, main_chunks[1]);

    // Body: file list + optional sidebar
    let body_area = main_chunks[2];
    let show_sidebar = area.width >= 100;

    if show_sidebar {
        let body_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(78), Constraint::Percentage(22)])
            .split(body_area);
        let list_height = body_chunks[0].height as usize;
        app.viewport_height = list_height;
        app.ensure_cursor_visible();
        list::render(f, app, body_chunks[0]);
        sidebar::render(f, app, body_chunks[1]);
    } else {
        let list_height = body_area.height as usize;
        app.viewport_height = list_height;
        app.ensure_cursor_visible();
        list::render(f, app, body_area);
    }

    // Fuzzy search overlay
    if app.mode == Mode::FuzzySearch {
        fuzzy::render(f, app, main_chunks[2]);
    }

    // Footer
    footer::render(f, app, main_chunks[3]);

    // Bookmark popup (renders on top of everything)
    if app.mode == Mode::Bookmark {
        bookmark::render(f, app, area);
    }

    // Theme picker popup
    if app.mode == Mode::ThemePicker {
        theme_picker::render(f, app, area);
    }
}
