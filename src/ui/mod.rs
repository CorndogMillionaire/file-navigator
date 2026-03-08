mod bookmark;
mod breadcrumb;
mod footer;
mod fuzzy;
mod header;
mod jumpkey;
mod list;
pub mod segment;
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

    // Body layout: 3 miller panels + optional sidebar
    //
    // Wide (140+):   [parent 18] [center flex] [child 22] [sidebar 20%]
    // Medium (100+): [parent 18] [center flex] [child 22]
    // Narrow (<100): [center only]
    let body_area = main_chunks[2];
    let total_width = area.width;

    let show_segments = total_width >= 100;
    let has_parent = !app.parent_entries.is_empty();
    let has_child = !app.child_preview.is_empty();
    // Only show sidebar at very wide terminals alongside segments
    let show_sidebar = total_width >= 160;

    let in_jump_mode = app.mode == Mode::JumpKey;
    let dim_pal = app.palette.dimmed(0.5);

    if show_segments {
        let mut constraints: Vec<Constraint> = Vec::new();
        let mut panel_order: Vec<PanelKind> = Vec::new();

        // Left: parent segment
        if has_parent {
            let parent_width = if total_width >= 140 { 22 } else { 18 };
            constraints.push(Constraint::Length(parent_width));
            panel_order.push(PanelKind::Parent);
        }

        // Center: main file list (always present)
        constraints.push(Constraint::Min(30));
        panel_order.push(PanelKind::Center);

        // Right: child preview
        if has_child {
            let child_width = if total_width >= 140 { 24 } else { 20 };
            constraints.push(Constraint::Length(child_width));
            panel_order.push(PanelKind::Child);
        }

        // Sidebar (only at very wide terminals)
        if show_sidebar {
            constraints.push(Constraint::Length((total_width as f32 * 0.15) as u16));
            panel_order.push(PanelKind::Sidebar);
        }

        let body_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(constraints)
            .split(body_area);

        for (chunk_idx, panel) in panel_order.iter().enumerate() {
            match panel {
                PanelKind::Parent => {
                    let jump_keys = if in_jump_mode { &app.parent_jump_keys } else { &[] as &[(usize, (char, char))] };
                    let pending = if in_jump_mode { app.pending_jump_key } else { None };
                    segment::render(
                        f,
                        &app.parent_entries,
                        app.parent_highlight,
                        &dim_pal,
                        &app.symbols,
                        body_chunks[chunk_idx],
                        false, // border on right
                        jump_keys,
                        pending,
                    );
                }
                PanelKind::Center => {
                    let list_height = body_chunks[chunk_idx].height as usize;
                    app.viewport_height = list_height;
                    app.ensure_cursor_visible();
                    list::render(f, app, body_chunks[chunk_idx]);
                }
                PanelKind::Child => {
                    let jump_keys = if in_jump_mode { &app.child_jump_keys } else { &[] as &[(usize, (char, char))] };
                    let pending = if in_jump_mode { app.pending_jump_key } else { None };
                    segment::render(
                        f,
                        &app.child_preview,
                        None,
                        &dim_pal,
                        &app.symbols,
                        body_chunks[chunk_idx],
                        true, // border on left
                        jump_keys,
                        pending,
                    );
                }
                PanelKind::Sidebar => {
                    sidebar::render(f, app, body_chunks[chunk_idx]);
                }
            }
        }
    } else {
        // Narrow terminal: file list only
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

enum PanelKind {
    Parent,
    Center,
    Child,
    Sidebar,
}
