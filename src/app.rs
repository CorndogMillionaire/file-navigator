use std::collections::HashMap;
use std::path::PathBuf;
use std::time::{Instant, SystemTime};

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;

use crate::config;
use crate::marks;
use crate::nav;
use crate::palette::{Palette, PALETTE_NAMES};
use crate::symbols::{SymbolSet, SYMBOL_SET_NAMES};

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Mode {
    Normal,
    FuzzySearch,
    JumpKey,
    Bookmark,
    ThemePicker,
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum ThemePickerFocus {
    Colors,
    Symbols,
}

pub struct FsEntry {
    pub name: String,
    pub path: PathBuf,
    pub is_dir: bool,
    pub size: Option<u64>,
    pub modified: Option<SystemTime>,
    pub permissions: Option<String>,
    pub depth: i32,
}

pub struct App {
    pub current_dir: PathBuf,
    pub entries: Vec<FsEntry>,
    pub cursor: usize,
    pub scroll_offset: usize,
    pub mode: Mode,
    pub nav_history: Vec<PathBuf>,
    pub nav_history_cursor: usize,
    pub marks: HashMap<char, PathBuf>,
    pub fuzzy_query: String,
    pub filtered_indices: Vec<usize>,
    pub error: Option<(String, Instant)>,
    pub blink_on: bool,
    pub last_blink: Instant,
    pub palette: Palette,
    pub palette_index: usize,
    pub symbols: SymbolSet,
    pub symbols_index: usize,
    pub should_quit: bool,
    pub selected_path: Option<PathBuf>,
    pub show_hidden: bool,
    pub last_position: Option<PathBuf>,
    pub jump_keys: Vec<(usize, (char, char))>,
    pub pending_jump_key: Option<char>,
    pub viewport_height: usize,
    // Deep fuzzy: entries from multiple depths
    pub fuzzy_pool: Vec<FsEntry>,
    pub fuzzy_filtered: Vec<usize>,
    // Bookmark popup state
    pub bookmark_query: String,
    pub bookmark_cursor: usize,
    pub bookmark_filtered: Vec<char>,
    // Theme picker state
    pub theme_picker_focus: ThemePickerFocus,
    pub theme_picker_color_cursor: usize,
    pub theme_picker_symbol_cursor: usize,
    // Saved state for cancel
    theme_picker_prev_palette: usize,
    theme_picker_prev_symbols: usize,
}

const JUMP_KEYS: &[(char, char)] = &[
    ('f', 'f'), ('f', 'd'), ('f', 's'), ('f', 'a'), ('f', 'g'),
    ('d', 'f'), ('d', 'd'), ('d', 's'), ('d', 'a'), ('d', 'g'),
    ('s', 'f'), ('s', 'd'), ('s', 's'), ('s', 'a'), ('s', 'g'),
    ('a', 'f'), ('a', 'd'), ('a', 's'), ('a', 'a'), ('a', 'g'),
    ('f', 'r'), ('d', 'r'), ('s', 'r'), ('a', 'r'),
    ('f', 'e'), ('d', 'e'), ('s', 'e'), ('a', 'e'),
    ('f', 'w'), ('d', 'w'), ('s', 'w'), ('a', 'w'),
    ('f', 'q'), ('d', 'q'), ('s', 'q'), ('a', 'q'),
    ('f', 'v'), ('d', 'v'), ('s', 'v'), ('a', 'v'),
    ('f', 'c'), ('d', 'c'), ('s', 'c'), ('a', 'c'),
    ('f', 'x'), ('d', 'x'), ('s', 'x'), ('a', 'x'),
    ('f', 'z'), ('d', 'z'), ('s', 'z'), ('a', 'z'),
];

impl App {
    pub fn new(
        palette: Palette,
        palette_index: usize,
        symbols: SymbolSet,
        symbols_index: usize,
        start_dir: Option<PathBuf>,
    ) -> Self {
        let current_dir = start_dir
            .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from("/")));
        let marks = marks::load_marks();
        let mut app = Self {
            current_dir: current_dir.clone(),
            entries: Vec::new(),
            cursor: 0,
            scroll_offset: 0,
            mode: Mode::Normal,
            nav_history: vec![current_dir],
            nav_history_cursor: 0,
            marks,
            fuzzy_query: String::new(),
            filtered_indices: Vec::new(),
            error: None,
            blink_on: true,
            last_blink: Instant::now(),
            palette,
            palette_index,
            symbols,
            symbols_index,
            should_quit: false,
            selected_path: None,
            show_hidden: false,
            last_position: None,
            jump_keys: Vec::new(),
            pending_jump_key: None,
            viewport_height: 0,
            fuzzy_pool: Vec::new(),
            fuzzy_filtered: Vec::new(),
            bookmark_query: String::new(),
            bookmark_cursor: 0,
            bookmark_filtered: Vec::new(),
            theme_picker_focus: ThemePickerFocus::Colors,
            theme_picker_color_cursor: palette_index,
            theme_picker_symbol_cursor: symbols_index,
            theme_picker_prev_palette: palette_index,
            theme_picker_prev_symbols: symbols_index,
        };
        app.refresh_entries();
        app
    }

    pub fn tick(&mut self) {
        if self.last_blink.elapsed().as_millis() >= 550 {
            self.blink_on = !self.blink_on;
            self.last_blink = Instant::now();
        }
        if let Some((_, timestamp)) = &self.error {
            if timestamp.elapsed().as_secs() >= 3 {
                self.error = None;
            }
        }
    }

    pub fn refresh_entries(&mut self) {
        match nav::read_dir(&self.current_dir, self.show_hidden) {
            Ok(entries) => self.entries = entries,
            Err(e) => {
                self.error = Some((format!("{}", e).to_uppercase(), Instant::now()));
                self.entries = Vec::new();
            }
        }
        self.filtered_indices = (0..self.entries.len()).collect();
        if self.cursor >= self.entries.len() {
            self.cursor = self.entries.len().saturating_sub(1);
        }
    }

    pub fn navigate_to(&mut self, path: PathBuf) {
        self.last_position = Some(self.current_dir.clone());
        self.current_dir = path.clone();
        self.cursor = 0;
        self.scroll_offset = 0;
        self.refresh_entries();
        self.nav_history.truncate(self.nav_history_cursor + 1);
        self.nav_history.push(path);
        self.nav_history_cursor = self.nav_history.len() - 1;
    }

    pub fn navigate_back(&mut self) {
        if self.nav_history_cursor > 0 {
            self.nav_history_cursor -= 1;
            let path = self.nav_history[self.nav_history_cursor].clone();
            self.last_position = Some(self.current_dir.clone());
            self.current_dir = path;
            self.cursor = 0;
            self.scroll_offset = 0;
            self.refresh_entries();
        }
    }

    pub fn navigate_forward(&mut self) {
        if self.nav_history_cursor + 1 < self.nav_history.len() {
            self.nav_history_cursor += 1;
            let path = self.nav_history[self.nav_history_cursor].clone();
            self.last_position = Some(self.current_dir.clone());
            self.current_dir = path;
            self.cursor = 0;
            self.scroll_offset = 0;
            self.refresh_entries();
        }
    }

    pub fn navigate_parent(&mut self) {
        if let Some(parent) = self.current_dir.parent() {
            let parent = parent.to_path_buf();
            self.navigate_to(parent);
        }
    }

    pub fn current_entry(&self) -> Option<&FsEntry> {
        if self.mode == Mode::FuzzySearch && !self.fuzzy_query.is_empty() {
            self.fuzzy_filtered
                .get(self.cursor)
                .and_then(|&i| self.fuzzy_pool.get(i))
        } else {
            self.filtered_indices
                .get(self.cursor)
                .and_then(|&i| self.entries.get(i))
        }
    }

    pub fn display_entries(&self) -> (&[FsEntry], &[usize]) {
        if self.mode == Mode::FuzzySearch && !self.fuzzy_query.is_empty() {
            (&self.fuzzy_pool, &self.fuzzy_filtered)
        } else {
            (&self.entries, &self.filtered_indices)
        }
    }

    pub fn build_fuzzy_pool(&mut self) {
        self.fuzzy_pool.clear();

        if let Ok(entries) = nav::read_dir(&self.current_dir, self.show_hidden) {
            for mut e in entries {
                e.depth = 0;
                self.fuzzy_pool.push(e);
            }
        }

        let sub_entries =
            nav::read_dir_recursive(&self.current_dir, self.show_hidden, 2, &self.current_dir);
        for e in sub_entries {
            if e.depth > 0 {
                self.fuzzy_pool.push(e);
            }
        }

        let parent_entries = nav::read_parent_entries(&self.current_dir, self.show_hidden);
        for e in parent_entries {
            self.fuzzy_pool.push(e);
        }

        self.fuzzy_filtered = (0..self.fuzzy_pool.len()).collect();
    }

    pub fn update_fuzzy_filter(&mut self) {
        if self.fuzzy_query.is_empty() {
            self.filtered_indices = (0..self.entries.len()).collect();
            self.fuzzy_filtered = (0..self.fuzzy_pool.len()).collect();
        } else {
            let matcher = SkimMatcherV2::default();
            let mut scored: Vec<(usize, i64)> = self
                .fuzzy_pool
                .iter()
                .enumerate()
                .filter_map(|(i, e)| {
                    matcher
                        .fuzzy_match(&e.name, &self.fuzzy_query)
                        .map(|score| (i, score))
                })
                .collect();
            scored.sort_by(|a, b| b.1.cmp(&a.1));
            self.fuzzy_filtered = scored.into_iter().map(|(i, _)| i).collect();

            let mut main_scored: Vec<(usize, i64)> = self
                .entries
                .iter()
                .enumerate()
                .filter_map(|(i, e)| {
                    matcher
                        .fuzzy_match(&e.name, &self.fuzzy_query)
                        .map(|score| (i, score))
                })
                .collect();
            main_scored.sort_by(|a, b| b.1.cmp(&a.1));
            self.filtered_indices = main_scored.into_iter().map(|(i, _)| i).collect();
        }
        if self.cursor >= self.fuzzy_filtered.len() {
            self.cursor = self.fuzzy_filtered.len().saturating_sub(1);
        }
    }

    pub fn assign_jump_keys(&mut self) {
        self.jump_keys.clear();
        self.pending_jump_key = None;
        let visible = self.visible_indices_in_viewport();
        for (key_idx, &entry_idx) in visible.iter().enumerate() {
            if key_idx < JUMP_KEYS.len() {
                self.jump_keys.push((entry_idx, JUMP_KEYS[key_idx]));
            }
        }
    }

    fn visible_indices_in_viewport(&self) -> Vec<usize> {
        let start = self.scroll_offset;
        let end = (start + self.viewport_height).min(self.filtered_indices.len());
        if start >= self.filtered_indices.len() {
            return Vec::new();
        }
        self.filtered_indices[start..end].to_vec()
    }

    pub fn jump_key_for_entry(&self, entry_idx: usize) -> Option<(char, char)> {
        self.jump_keys
            .iter()
            .find(|(idx, _)| *idx == entry_idx)
            .map(|(_, k)| *k)
    }

    pub fn entry_for_jump_key(&self, k: (char, char)) -> Option<usize> {
        self.jump_keys
            .iter()
            .find(|(_, key)| *key == k)
            .map(|(idx, _)| *idx)
    }

    pub fn ensure_cursor_visible(&mut self) {
        if self.viewport_height == 0 {
            return;
        }
        let total = if self.mode == Mode::FuzzySearch && !self.fuzzy_query.is_empty() {
            self.fuzzy_filtered.len()
        } else {
            self.filtered_indices.len()
        };
        if self.cursor >= total {
            self.cursor = total.saturating_sub(1);
        }
        if self.cursor < self.scroll_offset {
            self.scroll_offset = self.cursor;
        } else if self.cursor >= self.scroll_offset + self.viewport_height {
            self.scroll_offset = self.cursor - self.viewport_height + 1;
        }
    }

    pub fn palette_name(&self) -> &str {
        PALETTE_NAMES[self.palette_index]
    }

    pub fn symbol_set_name(&self) -> &str {
        SYMBOL_SET_NAMES[self.symbols_index]
    }

    fn open_theme_picker(&mut self) {
        self.mode = Mode::ThemePicker;
        self.theme_picker_focus = ThemePickerFocus::Colors;
        self.theme_picker_color_cursor = self.palette_index;
        self.theme_picker_symbol_cursor = self.symbols_index;
        self.theme_picker_prev_palette = self.palette_index;
        self.theme_picker_prev_symbols = self.symbols_index;
    }

    fn apply_theme_selection(&mut self) {
        self.palette_index = self.theme_picker_color_cursor;
        self.palette = Palette::from_name(PALETTE_NAMES[self.palette_index]);
        self.symbols_index = self.theme_picker_symbol_cursor;
        self.symbols = SymbolSet::from_name(SYMBOL_SET_NAMES[self.symbols_index]);
        config::save_config(self.palette_name(), self.symbol_set_name());
    }

    // Bookmark helpers
    fn open_bookmark_popup(&mut self) {
        self.mode = Mode::Bookmark;
        self.bookmark_query.clear();
        self.bookmark_cursor = 0;
        self.update_bookmark_filter();
    }

    fn update_bookmark_filter(&mut self) {
        let mut keys: Vec<char> = self.marks.keys().copied().collect();
        keys.sort();
        if self.bookmark_query.is_empty() {
            self.bookmark_filtered = keys;
        } else {
            let matcher = SkimMatcherV2::default();
            let mut scored: Vec<(char, i64)> = keys
                .into_iter()
                .filter_map(|k| {
                    let path = &self.marks[&k];
                    let name = path.to_string_lossy().to_string();
                    matcher
                        .fuzzy_match(&name, &self.bookmark_query)
                        .map(|score| (k, score))
                })
                .collect();
            scored.sort_by(|a, b| b.1.cmp(&a.1));
            self.bookmark_filtered = scored.into_iter().map(|(k, _)| k).collect();
        }
        if self.bookmark_cursor >= self.bookmark_filtered.len() {
            self.bookmark_cursor = self.bookmark_filtered.len().saturating_sub(1);
        }
    }

    fn add_bookmark(&mut self) {
        // Auto-assign next available letter
        let used: std::collections::HashSet<char> = self.marks.keys().copied().collect();
        let next_key = ('a'..='z').find(|c| !used.contains(c));
        if let Some(key) = next_key {
            self.marks.insert(key, self.current_dir.clone());
            marks::save_marks(&self.marks);
            self.error = Some((
                format!("BOOKMARK [{}] SET: {}", key, self.current_dir.display()),
                Instant::now(),
            ));
        } else {
            self.error = Some(("ALL BOOKMARK SLOTS FULL".to_string(), Instant::now()));
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        if self.error.is_some() {
            self.error = None;
        }

        match self.mode {
            Mode::Normal => self.handle_normal_key(key),
            Mode::FuzzySearch => self.handle_fuzzy_key(key),
            Mode::JumpKey => self.handle_jump_key(key),
            Mode::Bookmark => self.handle_bookmark_key(key),
            Mode::ThemePicker => self.handle_theme_picker_key(key),
        }
    }

    fn handle_normal_key(&mut self, key: KeyEvent) {
        match (key.code, key.modifiers) {
            (KeyCode::Char('q'), KeyModifiers::NONE) => {
                self.should_quit = true;
            }
            (KeyCode::Esc, _) => {
                self.should_quit = true;
            }
            // Movement
            (KeyCode::Char('j') | KeyCode::Down, _) => {
                if self.cursor + 1 < self.filtered_indices.len() {
                    self.cursor += 1;
                    self.ensure_cursor_visible();
                }
            }
            (KeyCode::Char('k') | KeyCode::Up, _) => {
                if self.cursor > 0 {
                    self.cursor -= 1;
                    self.ensure_cursor_visible();
                }
            }
            (KeyCode::Char('G'), KeyModifiers::SHIFT) | (KeyCode::Char('G'), _) => {
                if !self.filtered_indices.is_empty() {
                    self.cursor = self.filtered_indices.len() - 1;
                    self.ensure_cursor_visible();
                }
            }
            (KeyCode::Char('u'), KeyModifiers::CONTROL) => {
                let half = self.viewport_height / 2;
                self.cursor = self.cursor.saturating_sub(half);
                self.ensure_cursor_visible();
            }
            (KeyCode::Char('d'), KeyModifiers::CONTROL) => {
                let half = self.viewport_height / 2;
                self.cursor =
                    (self.cursor + half).min(self.filtered_indices.len().saturating_sub(1));
                self.ensure_cursor_visible();
            }
            // Navigation
            (KeyCode::Char('h') | KeyCode::Left, _)
                if key.modifiers == KeyModifiers::NONE || key.code == KeyCode::Left =>
            {
                self.navigate_parent();
            }
            (KeyCode::Char('-'), KeyModifiers::NONE) => {
                self.navigate_parent();
            }
            (KeyCode::Char('l') | KeyCode::Right, _) => {
                self.enter_selected();
            }
            (KeyCode::Enter, _) => {
                self.enter_selected();
            }
            (KeyCode::Char('o'), KeyModifiers::CONTROL) => {
                self.navigate_back();
            }
            (KeyCode::Char('i'), KeyModifiers::CONTROL) => {
                self.navigate_forward();
            }
            // Stay: return PWD
            (KeyCode::Char('s'), KeyModifiers::NONE) => {
                self.selected_path = Some(self.current_dir.clone());
            }
            // Goto: return selected directory
            (KeyCode::Char('g'), KeyModifiers::NONE) => {
                if let Some(entry) = self.current_entry() {
                    if entry.is_dir {
                        self.selected_path = Some(entry.path.clone());
                    }
                }
            }
            // Modes
            (KeyCode::Char('/'), KeyModifiers::NONE) => {
                self.mode = Mode::FuzzySearch;
                self.fuzzy_query.clear();
                self.cursor = 0;
                self.scroll_offset = 0;
                self.build_fuzzy_pool();
            }
            (KeyCode::Char(' '), KeyModifiers::NONE) => {
                self.mode = Mode::JumpKey;
                self.assign_jump_keys();
            }
            // Bookmarks
            (KeyCode::Char('b'), KeyModifiers::NONE) => {
                if self.marks.is_empty() {
                    self.error = Some((
                        "NO BOOKMARKS. PRESS B TO ADD ONE.".to_string(),
                        Instant::now(),
                    ));
                } else {
                    self.open_bookmark_popup();
                }
            }
            (KeyCode::Char('B'), _) => {
                self.add_bookmark();
            }
            // Theme picker
            (KeyCode::Char('t'), KeyModifiers::NONE) => {
                self.open_theme_picker();
            }
            // Hidden files toggle
            (KeyCode::Char('.'), KeyModifiers::NONE) => {
                self.show_hidden = !self.show_hidden;
                self.refresh_entries();
            }
            _ => {}
        }
    }

    fn handle_fuzzy_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Esc => {
                self.mode = Mode::Normal;
                self.fuzzy_query.clear();
                self.filtered_indices = (0..self.entries.len()).collect();
                self.cursor = 0;
                self.scroll_offset = 0;
            }
            KeyCode::Enter => {
                if let Some(entry) = self.current_entry() {
                    let path = entry.path.clone();
                    let is_dir = entry.is_dir;
                    self.mode = Mode::Normal;
                    self.fuzzy_query.clear();
                    if is_dir {
                        self.navigate_to(path);
                    } else {
                        if let Some(parent) = path.parent() {
                            self.navigate_to(parent.to_path_buf());
                        }
                    }
                } else {
                    self.mode = Mode::Normal;
                    self.fuzzy_query.clear();
                    self.filtered_indices = (0..self.entries.len()).collect();
                    self.cursor = 0;
                }
            }
            KeyCode::Backspace => {
                self.fuzzy_query.pop();
                self.cursor = 0;
                self.scroll_offset = 0;
                self.update_fuzzy_filter();
            }
            KeyCode::Down | KeyCode::Tab => {
                let total = self.fuzzy_filtered.len();
                if self.cursor + 1 < total {
                    self.cursor += 1;
                    self.ensure_cursor_visible();
                }
            }
            KeyCode::Up | KeyCode::BackTab => {
                if self.cursor > 0 {
                    self.cursor -= 1;
                    self.ensure_cursor_visible();
                }
            }
            KeyCode::Char(c) => {
                self.fuzzy_query.push(c);
                self.cursor = 0;
                self.scroll_offset = 0;
                self.update_fuzzy_filter();
            }
            _ => {}
        }
    }

    fn handle_jump_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char(c) if c.is_ascii_lowercase() => {
                if let Some(first) = self.pending_jump_key.take() {
                    // Second key of the combo
                    if let Some(entry_idx) = self.entry_for_jump_key((first, c)) {
                        if let Some(cursor_pos) =
                            self.filtered_indices.iter().position(|&i| i == entry_idx)
                        {
                            self.cursor = cursor_pos;
                            self.ensure_cursor_visible();
                            self.mode = Mode::Normal;
                            self.enter_selected();
                            return;
                        }
                    }
                    self.mode = Mode::Normal;
                } else {
                    // First key - check if any jump key starts with this char
                    if self.jump_keys.iter().any(|(_, (k1, _))| *k1 == c) {
                        self.pending_jump_key = Some(c);
                    } else {
                        self.mode = Mode::Normal;
                    }
                }
            }
            KeyCode::Esc => {
                self.pending_jump_key = None;
                self.mode = Mode::Normal;
            }
            _ => {
                self.pending_jump_key = None;
                self.mode = Mode::Normal;
            }
        }
    }

    fn handle_bookmark_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Esc => {
                self.mode = Mode::Normal;
            }
            KeyCode::Char('h') if key.modifiers == KeyModifiers::NONE => {
                self.mode = Mode::Normal;
            }
            KeyCode::Char('l') if key.modifiers == KeyModifiers::NONE => {
                if let Some(&mark_key) = self.bookmark_filtered.get(self.bookmark_cursor) {
                    if let Some(path) = self.marks.get(&mark_key).cloned() {
                        self.mode = Mode::Normal;
                        self.navigate_to(path);
                    }
                }
            }
            KeyCode::Enter => {
                if let Some(&mark_key) = self.bookmark_filtered.get(self.bookmark_cursor) {
                    if let Some(path) = self.marks.get(&mark_key).cloned() {
                        self.mode = Mode::Normal;
                        self.navigate_to(path);
                    }
                }
            }
            KeyCode::Backspace => {
                self.bookmark_query.pop();
                self.bookmark_cursor = 0;
                self.update_bookmark_filter();
            }
            KeyCode::Char('j') if key.modifiers == KeyModifiers::NONE => {
                if self.bookmark_cursor + 1 < self.bookmark_filtered.len() {
                    self.bookmark_cursor += 1;
                }
            }
            KeyCode::Down | KeyCode::Tab => {
                if self.bookmark_cursor + 1 < self.bookmark_filtered.len() {
                    self.bookmark_cursor += 1;
                }
            }
            KeyCode::Char('k') if key.modifiers == KeyModifiers::NONE => {
                if self.bookmark_cursor > 0 {
                    self.bookmark_cursor -= 1;
                }
            }
            KeyCode::Up | KeyCode::BackTab => {
                if self.bookmark_cursor > 0 {
                    self.bookmark_cursor -= 1;
                }
            }
            // Delete bookmark with ctrl+d
            KeyCode::Char('d') if key.modifiers == KeyModifiers::CONTROL => {
                if let Some(&mark_key) = self.bookmark_filtered.get(self.bookmark_cursor) {
                    self.marks.remove(&mark_key);
                    marks::save_marks(&self.marks);
                    self.update_bookmark_filter();
                    if self.bookmark_filtered.is_empty() {
                        self.mode = Mode::Normal;
                    }
                }
            }
            KeyCode::Char(c) => {
                self.bookmark_query.push(c);
                self.bookmark_cursor = 0;
                self.update_bookmark_filter();
            }
            _ => {}
        }
    }

    fn handle_theme_picker_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Esc | KeyCode::Char('q') => {
                // Restore previous theme on cancel
                self.palette_index = self.theme_picker_prev_palette;
                self.palette = Palette::from_name(PALETTE_NAMES[self.palette_index]);
                self.symbols_index = self.theme_picker_prev_symbols;
                self.symbols = SymbolSet::from_name(SYMBOL_SET_NAMES[self.symbols_index]);
                self.mode = Mode::Normal;
            }
            // Switch focus between tables
            KeyCode::Tab | KeyCode::Char('h') | KeyCode::Char('l') | KeyCode::Left | KeyCode::Right => {
                self.theme_picker_focus = match self.theme_picker_focus {
                    ThemePickerFocus::Colors => ThemePickerFocus::Symbols,
                    ThemePickerFocus::Symbols => ThemePickerFocus::Colors,
                };
            }
            // Navigate within focused table
            KeyCode::Char('j') | KeyCode::Down => {
                match self.theme_picker_focus {
                    ThemePickerFocus::Colors => {
                        if self.theme_picker_color_cursor + 1 < PALETTE_NAMES.len() {
                            self.theme_picker_color_cursor += 1;
                        }
                    }
                    ThemePickerFocus::Symbols => {
                        if self.theme_picker_symbol_cursor + 1 < SYMBOL_SET_NAMES.len() {
                            self.theme_picker_symbol_cursor += 1;
                        }
                    }
                }
                // Live preview
                self.palette_index = self.theme_picker_color_cursor;
                self.palette = Palette::from_name(PALETTE_NAMES[self.palette_index]);
                self.symbols_index = self.theme_picker_symbol_cursor;
                self.symbols = SymbolSet::from_name(SYMBOL_SET_NAMES[self.symbols_index]);
            }
            KeyCode::Char('k') | KeyCode::Up => {
                match self.theme_picker_focus {
                    ThemePickerFocus::Colors => {
                        if self.theme_picker_color_cursor > 0 {
                            self.theme_picker_color_cursor -= 1;
                        }
                    }
                    ThemePickerFocus::Symbols => {
                        if self.theme_picker_symbol_cursor > 0 {
                            self.theme_picker_symbol_cursor -= 1;
                        }
                    }
                }
                // Live preview
                self.palette_index = self.theme_picker_color_cursor;
                self.palette = Palette::from_name(PALETTE_NAMES[self.palette_index]);
                self.symbols_index = self.theme_picker_symbol_cursor;
                self.symbols = SymbolSet::from_name(SYMBOL_SET_NAMES[self.symbols_index]);
            }
            // Confirm selection
            KeyCode::Enter => {
                self.apply_theme_selection();
                self.mode = Mode::Normal;
            }
            _ => {}
        }
    }

    fn enter_selected(&mut self) {
        if let Some(entry) = self.current_entry() {
            let path = entry.path.clone();
            let is_dir = entry.is_dir;
            if is_dir {
                self.navigate_to(path);
            }
        }
    }

    pub fn fuzzy_match_indices(&self, text: &str) -> Vec<usize> {
        if self.fuzzy_query.is_empty() {
            return Vec::new();
        }
        let matcher = SkimMatcherV2::default();
        matcher
            .fuzzy_indices(text, &self.fuzzy_query)
            .map(|(_, indices)| indices)
            .unwrap_or_default()
    }
}
