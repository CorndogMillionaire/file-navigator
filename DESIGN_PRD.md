# DESIGN PRD — FILE NAVIGATOR TUI
## Codename: `REM` (Remote Entry Module)

---

## 1. Vision

REM is a small, sharp tool. It does one thing — navigate a file tree and emit a path — and it does it with zero friction. The design philosophy is:

- **Utility over features.** Every interaction exists because it makes navigation faster. Nothing decorative that doesn't serve the task.
- **Keyboard ergonomics first.** Every keybinding is chosen for physical comfort. Homerow keys dominate. Two-key jumps keep fingers centered. Modes are fast to enter and fast to exit.
- **Looks cool as fuck.** The aesthetic is late-70s/early-80s sci-fi — phosphor CRTs, degraded signal, corporate bureaucracy, mining terminals. Think *Alien* (1979). The tool should feel like it was built by a corporation that doesn't care about you, running on hardware that's seen better decades.
- **Fast.** Instant startup, instant response, no loading spinners, no delays. Every frame is a full redraw at 10fps. The tool should feel like it's already done before you finished thinking.

REM is not a file manager. It doesn't copy, move, rename, or delete. It navigates and emits. That's it.

---

## 2. Design Philosophy

### 2.1 Core Principles

**Phosphor first.** Every element reads as light emitted from a phosphor-coated screen. Colors have glow, falloff, and self-illumination. Implemented through layered brightness: a dim base, a mid tone, and a hot highlight for selected/active states.

**Scanlines are structural.** Text never feels cramped. Rows have breathing room because phosphor CRTs needed it. One blank line between major sections, consistent vertical rhythm.

**Corporate ugly.** The UI was spec'd by a mining corporation in 2122. Labels are terse, uppercase, bureaucratic. Errors are clinical. No "friendly" language. No rounded corners. No softness.

**Degradation is honest.** The terminal has been used. Signal degrades. Things flicker. Status indicators exist because things fail.

**Output is the product.** The navigator exists to emit a path and exit. Every UI decision reduces friction toward that goal.

### 2.2 Anti-patterns

- No color gradients
- No rounded border styles — `Plain` borders only
- No friendly microcopy ("Oops!", "Nothing here yet")
- No centered layouts — left-aligned or grid-aligned
- No pastel or desaturated colors — phosphor colors are vivid against dark

---

## 3. Color System

Five palettes, each representing a different terminal "unit type." Default is Phosphor Green. Users switch palettes via the theme picker (`t`) or config file.

### 3.1 Nine-Color Architecture

Every palette defines exactly nine colors:

```
BACKGROUND    — near black, not pure
SURFACE       — slightly elevated (hovered rows, bars)
BORDER_DIM    — barely visible
BORDER_MID    — structural separators
BORDER_HOT    — active / focused
TEXT_DIM      — readable but recessed
TEXT_MID      — default body text
TEXT_HOT      — selected, active, cursor
WARN          — errors and warnings only
```

Pass `&Palette` into every render function. Never hardcode colors in render logic.

### 3.2 Palette Definitions

**Phosphor Green** (Default) — *The mainframe. Institutional. Cold.*
```rust
bg: Rgb(3, 3, 3), surface: Rgb(2, 12, 2),
text_dim: Rgb(0, 82, 24), text_mid: Rgb(0, 168, 40), text_hot: Rgb(0, 255, 65),
border_dim: Rgb(0, 26, 8), border_mid: Rgb(0, 61, 16), border_hot: Rgb(0, 122, 34),
warn: Rgb(255, 68, 68)
```

**Amber Corporate** — *The executive terminal. Weyland-Yutani ops. Every access is logged.*
```rust
bg: Rgb(12, 8, 0), surface: Rgb(17, 10, 0),
text_dim: Rgb(90, 58, 0), text_mid: Rgb(196, 122, 0), text_hot: Rgb(255, 176, 0),
border_dim: Rgb(58, 40, 0), border_mid: Rgb(107, 74, 0), border_hot: Rgb(128, 88, 0),
warn: Rgb(255, 68, 68)
```

**Degraded Cyan** — *The field unit. Dropped one too many times. Still works. Barely.*
```rust
bg: Rgb(1, 10, 13), surface: Rgb(1, 13, 16),
text_dim: Rgb(0, 96, 112), text_mid: Rgb(0, 149, 168), text_hot: Rgb(0, 229, 255),
border_dim: Rgb(0, 21, 32), border_mid: Rgb(0, 48, 64), border_hot: Rgb(0, 96, 122),
warn: Rgb(255, 68, 68)
```

**Crimson Red** — *Emergency lighting.*
```rust
bg: Rgb(10, 2, 2), surface: Rgb(14, 3, 3),
text_dim: Rgb(100, 18, 18), text_mid: Rgb(180, 30, 30), text_hot: Rgb(255, 50, 50),
border_dim: Rgb(40, 6, 6), border_mid: Rgb(80, 14, 14), border_hot: Rgb(120, 20, 20),
warn: Rgb(255, 200, 60)
```

**Hot Pink** — *Corrupted signal.*
```rust
bg: Rgb(10, 2, 8), surface: Rgb(14, 3, 11),
text_dim: Rgb(100, 18, 80), text_mid: Rgb(200, 40, 160), text_hot: Rgb(255, 80, 200),
border_dim: Rgb(40, 6, 30), border_mid: Rgb(80, 14, 60), border_hot: Rgb(140, 25, 100),
warn: Rgb(255, 200, 60)
```

Each palette also carries a corporate ASCII art logo displayed in the sidebar.

---

## 4. Symbol Sets

REM uses a configurable symbol set system for all glyphs and sigils. Five symbol sets ship by default, selectable via the theme picker (`t`, right column) or config file. This keeps the visual language consistent while supporting different terminals and aesthetics.

### 4.1 Symbol Set Architecture

Every symbol set defines:

```
dir_sigil         — directory marker (▣, #, ■, ●, ◉)
file_sigil        — file marker (◻, -, □, ○, ◎)
cursor            — selection indicator (▶, >, ►, ▸, →)
blink_char        — blinking cursor (▋, _, █, │, ▏)
depth_up          — fuzzy depth: parent (↑, ^, ⬆, ˄, △)
depth_down        — fuzzy depth: child (↓, v, ⬇, ˅, ▽)
separator         — dot separator in headers (·, ., •, ‧, │)
em_dash           — size placeholder for dirs (—, --, ━, –, ─)
warning           — error indicator (⚠, !, ◆, ×, ⊗)
bookmark_icon     — bookmark marker (⚑, *, ◈, †, ⌂)
scrollbar_thumb   — scrollbar filled (█, #, ┃, │, ▐)
scrollbar_track   — scrollbar empty (│, |, ╎, ┆, ░)
horizontal_rule   — section divider (─, -, ━, ·, ╌)
ellipsis          — truncation (…, .., ⋯, ~, ⸱⸱)
nav_arrows        — breadcrumb seps (/, /, /, /, ›)
```

### 4.2 Available Symbol Sets

| Set | Vibe | Terminal Requirements |
|---|---|---|
| **Standard** | Unicode box-drawing, the default | Full Unicode |
| **ASCII** | Pure ASCII, maximum compatibility | Any terminal |
| **Block** | Heavy Unicode blocks, bold feel | Full Unicode |
| **Minimal** | Elegant, sparse, quiet | Full Unicode |
| **Pipeline** | Arrow-based, inline flow | Full Unicode |

### 4.3 Planned: Permission Symbol Sets

Permissions are currently rendered as text (`rwxr-xr-x`) in the sidebar. The next evolution: **encode permissions into the symbol set system** so they can be rendered as compact, at-a-glance symbolic representations in the file list itself.

Design goals for permission symbols:
- Must be intuitive — readable without a legend after 5 minutes of use
- Must be compact — fit in a fixed-width column alongside the file list
- Must conform to the aesthetic — no emoji, no color-only encoding
- Should leverage the existing symbol set architecture (each set defines its own permission glyphs)

Possible approaches:
- **Symbolic triads**: Three-character groups per permission class (owner/group/other), using distinct glyphs for read/write/execute (e.g., `◉◉◉ ◉·· ◉··` for `rwx r-- r--`)
- **Density encoding**: Fill level of a single character per class (e.g., `█▄░` for rwx/r--/---)
- **Lock/unlock sigils**: Binary access indicators with distinct shapes per permission type

This feature will extend `SymbolSet` with permission-related fields and add a permissions column to the file list layout.

---

## 5. Typography & Text Conventions

### 5.1 Case Rules

| Context | Case |
|---|---|
| Section headers, labels | `UPPERCASE` |
| File and directory names | Preserve exact case |
| Status values | `UPPERCASE` |
| Key hint labels | lowercase (`hjkl move`) |
| Path display | Preserve exact case |
| Error messages | `UPPERCASE. TERSE.` |

### 5.2 Spacing Rhythm

```
Header bar height:        1 row
Section label padding:    0 left-pad, 1 row gap below
Row height:               1 row (no padding rows between entries)
Panel internal padding:   1 col left, 1 col right
Major section separation: 1 border line
Footer bar height:        1 row
```

### 5.3 Truncation

All text that may overflow truncates with `…` (from active symbol set). Never wrap. File names truncate on the right. Paths truncate on the left — show the tail, not the head.

---

## 6. Layout System

### 6.1 Primary Layout

```
+-----------------------------------------------------+
|  HEADER BAR                                          |  1 row
+-----------------------------------------------------+
|  BREADCRUMB / PATH BAR                               |  1 row
+------------------------------+-----------------------+
|                              |                       |
|  FILE LIST                   |  INFO SIDEBAR         |
|  (primary pane)              |  ~22% width           |
|                              |  (hides < 100 cols)   |
|                              |                       |
+------------------------------+-----------------------+
|  FOOTER / KEY HINTS                                  |  1 row
+-----------------------------------------------------+
```

### 6.2 Column Proportions

```
File list (no sidebar):   100% width
File list (with sidebar): 78% width
Sidebar:                  22% width

List row columns:
  selection:   2 cols  (cursor indicator or blank)
  jump key:    5 cols  ([ff] in jump mode, blank otherwise)
  sigil:       2 cols  (dir/file glyph)
  name:        flex    (takes remaining space)
  type badge:  6 cols  (right-aligned, hides < 80 cols)
  size:        9 cols  (right-aligned, hides < 90 cols)
```

### 6.3 Responsive Collapse

- Below 100 cols: hide the sidebar
- Below 90 cols: hide the size column
- Below 80 cols: hide size and type columns
- Name column never collapses below 20 chars

---

## 7. Components

### 7.1 Header Bar

Single row. Left: app identifier ("REM NAVIGATOR"). Right: item count, palette name.

- Background: `SURFACE`
- Identifier: `TEXT_HOT`, Bold
- Separator (from symbol set): `TEXT_DIM`
- Status values: `TEXT_HOT`

### 7.2 Breadcrumb / Path Bar

Current path as slash-separated segments with a trailing blinking cursor.

- Final segment: `TEXT_HOT`
- Parent segments: `TEXT_MID`
- Separators: `TEXT_DIM`
- Trailing cursor (from symbol set `blink_char`): blinks at 550ms interval

### 7.3 File List

The primary pane. Scrollable, sorted (directories first, then files, case-insensitive alphabetical).

**Row states:**

| State | Background | Text | Indicator |
|---|---|---|---|
| Default | `BG` | `TEXT_DIM` | none |
| Hovered (cursor) | `SURFACE` | `TEXT_HOT` | cursor sigil prepended |
| Fuzzy depth 0 | `BG` | `TEXT_MID` | none |
| Fuzzy depth +/-1 | `BG` | `TEXT_DIM` | depth arrow |
| Fuzzy depth 2+ | `BG` | `BORDER_MID` | depth arrow |

**Scrollbar:** 1-col on far right, thumb/track from symbol set, `BORDER_DIM`. Thumb size proportional to viewport/total ratio.

### 7.4 Info Sidebar

Right panel (visible at >= 100 cols). Shows:

1. **SELECTION** — name, type, size, permissions, modification date of hovered entry
2. **BOOKMARKS** — list of saved bookmarks with keys
3. **ASCII art logo** — palette-specific corporate branding

Section labels: `UPPERCASE`, `TEXT_DIM`. Values: `TEXT_HOT`.

### 7.5 Footer / Key Hints

Single row, context-aware — shows different hints per mode.

- Key glyphs: `TEXT_MID`
- Descriptions: `TEXT_DIM`
- Separator: `BORDER_MID`
- Background: `SURFACE`

### 7.6 Fuzzy Search Overlay

Activated by `/`. Inline row at the bottom of the file list.

**Deep search:** Searches across current directory (depth 0), subdirectories (up to 2 levels deep), and parent directory entries (depth -1). Results are colored by depth — current directory is brightest, deeper results progressively dimmer.

- Prompt: `TEXT_HOT`
- Input text + cursor: `TEXT_HOT`, cursor blinks
- Match count: right-aligned, `TEXT_DIM`
- Matching characters in results: Bold
- ESC cancels; Enter navigates to selected match
- Tab/BackTab and Up/Down navigate results

### 7.7 Two-Key Jump Overlay

Activated by `Space`. Assigns **two-character** key combinations to all visible entries (e.g., `ff`, `fd`, `fs`, `fa`, `fg`, `df`, `dd`, etc.). Homerow-first for ergonomics.

**Flow:**
1. Press Space — jump keys appear next to all visible entries
2. Press first character — entries that don't match dim out, remaining entries show their second character
3. Press second character — immediately navigate to and enter that entry

- Up to 44 visible entries can receive jump keys
- ESC cancels at any point
- Non-jump content dims to `TEXT_DIM`
- Jump key badges: `TEXT_HOT`, Bold

### 7.8 Bookmark Popup

Activated by `b`. Modal overlay showing all saved bookmarks.

- Searchable — type to filter bookmarks by path
- Navigate with `j`/`k` or arrows
- `l` or Enter: navigate to selected bookmark
- `Ctrl+D`: delete selected bookmark
- `h` or ESC: close popup
- `B` (in normal mode): auto-assign current directory to next available a-z slot

Bookmarks persist to `~/.config/rem/marks.toml`. Maximum 26 bookmarks (a-z).

### 7.9 Theme Picker Popup

Activated by `t`. Two-column modal selector.

- Left column: color palettes (5 options)
- Right column: symbol sets (5 options)
- Tab or `h`/`l` switches between columns
- `j`/`k` navigates within a column
- Live preview while browsing
- Enter: apply and save to config
- ESC: cancel and revert to previous theme

Selections persist to `~/.config/rem/config.toml`.

---

## 8. Interaction Model

### 8.1 Keybindings

**Normal Mode:**
```
h / <- / -      Go to parent directory
l / -> / Enter  Enter directory (or emit file path)
j / Down        Cursor down
k / Up          Cursor up
G               Jump to bottom of list
Ctrl+U          Scroll up half page
Ctrl+D          Scroll down half page
Ctrl+O          Navigate back in history
Ctrl+I          Navigate forward in history
s               Stay — emit current working directory and exit
g               Go — emit hovered directory and exit
Space           Two-key jump overlay
/               Fuzzy search
b               Bookmark popup
B               Add current directory as bookmark
t               Theme picker
.               Toggle hidden files
q / Esc         Quit (no output, exit code 1)
```

**Fuzzy Search Mode:**
```
Type            Filter matches
Up / Down       Navigate results
Tab / BackTab   Navigate results
Enter           Navigate to selected match
Esc             Cancel search
Backspace       Delete from query
```

**Jump Key Mode:**
```
[1st key]       Wait for second key (non-matches dim)
[2nd key]       Navigate to and enter target
Esc             Cancel
```

**Bookmark Popup:**
```
j / k / Arrows  Navigate bookmarks
l / Enter       Go to selected bookmark
h / Esc         Close popup
Ctrl+D          Delete selected bookmark
Type            Search within bookmarks
```

**Theme Picker:**
```
j / k / Arrows  Navigate current column
Tab / h / l     Switch between columns
Enter           Apply theme and save
Esc / q         Cancel and revert
```

### 8.2 Exit Behavior

| Action | Output | Exit Code |
|---|---|---|
| `s` (stay) | Current working directory path | 0 |
| `g` (go) | Hovered directory path | 0 |
| `l`/Enter on file | Navigate to containing dir, emit path | 0 |
| `q` / Esc | Nothing | 1 |

Shell integration wrapper captures stdout and `cd`s into it:

```bash
rem() {
  local result=$(command rem "$@")
  if [ $? -eq 0 ] && [ -n "$result" ]; then
    cd "$result" || return
  fi
}
```

Automatic setup: `rem --shell-init >> ~/.bashrc`

### 8.3 Navigation History

REM tracks a history stack of visited directories. `Ctrl+O` goes back, `Ctrl+I` goes forward — same mental model as vim's jumplist.

---

## 9. Animation & Timing

- **Tick rate:** 100ms (10fps)
- **Cursor blink:** 550ms toggle interval
- **Error auto-dismiss:** 3 seconds
- **No transitions.** Every frame is a full redraw. No fade, no slide, no easing.

---

## 10. Configuration

### 10.1 Config File

Location: `~/.config/rem/config.toml`

```toml
palette = "phosphor"    # phosphor, amber, cyan, red, pink
symbols = "standard"    # standard, ascii, block, minimal, pipeline
```

### 10.2 CLI Arguments

```
rem [OPTIONS]

--palette <NAME>    Override color palette
--symbols <NAME>    Override symbol set
--shell-init        Print shell wrapper function
--help              Show help
```

CLI args take priority over config file. Config file takes priority over defaults.

### 10.3 Bookmarks File

Location: `~/.config/rem/marks.toml`

```toml
[marks]
a = "/home/user/projects"
b = "/etc"
```

---

## 11. Architecture

### 11.1 File Structure

```
src/
  main.rs            — terminal setup/teardown, event loop, arg parsing
  app.rs             — App struct, Mode enum, key handlers, tick logic
  nav.rs             — directory read, recursive read, parent entries
  marks.rs           — bookmark persistence (TOML)
  config.rs          — config load/save (palette + symbol set)
  palette.rs         — 5 color palettes + ASCII art logos
  symbols.rs         — 5 symbol sets
  ui/
    mod.rs           — top-level layout (header/breadcrumb/body/footer)
    header.rs        — header bar
    breadcrumb.rs    — path bar with blinking cursor
    list.rs          — file list + scrollbar + depth colors + fuzzy highlighting
    sidebar.rs       — info panel: selection, bookmarks, logo
    footer.rs        — context-aware key hints
    fuzzy.rs         — fuzzy search overlay row
    jumpkey.rs       — jump key rendering
    bookmark.rs      — bookmark modal popup
    theme_picker.rs  — two-column theme/symbol selector
```

### 11.2 Dependencies

```toml
[dependencies]
ratatui       = "0.30"
crossterm     = "0.29"
fuzzy-matcher = "0.3"
serde         = { version = "1", features = ["derive"] }
toml          = "0.8"
dirs          = "6"
```

### 11.3 App State

```rust
pub struct App {
    pub current_dir: PathBuf,
    pub entries: Vec<FsEntry>,
    pub cursor: usize,
    pub scroll_offset: usize,
    pub mode: Mode,                      // Normal, FuzzySearch, JumpKey, Bookmark, ThemePicker
    pub nav_history: Vec<PathBuf>,
    pub nav_history_cursor: usize,
    pub marks: HashMap<char, PathBuf>,
    pub fuzzy_query: String,
    pub fuzzy_pool: Vec<FsEntry>,        // multi-depth entries for fuzzy search
    pub selected_path: Option<PathBuf>,  // set on successful exit
    pub should_quit: bool,
    pub blink_on: bool,
    pub show_hidden: bool,
    pub palette: Palette,
    pub symbols: SymbolSet,
    pub jump_keys: Vec<(String, usize)>, // two-char labels mapped to entry indices
    pub error: Option<(String, Instant)>,
}
```

---

## 12. Out of Scope

REM deliberately excludes:

- File operations (copy, move, rename, delete)
- File preview / content viewing
- Image rendering
- Plugin system
- Mouse support
- Git status indicators
- Tabs or split panes
- Networked / remote filesystems
- Tree view with expand/collapse (segments are flat lists, not hierarchical trees)

The tool does one thing: navigate a file tree ergonomically and emit a path.

---

## 13. Roadmap

### 13.1 Next: Fluid Tree Navigation (Miller Segments + Cursor Memory)

The single biggest ergonomic gap right now: navigating back out of a directory resets the cursor to the top. You lose your place. And when you're deep in a tree, you can't see the branches you came through or jump sideways into sibling paths. This feature closes both gaps and makes the navigator feel *fluid* — like you're sliding through the tree, not clicking through pages.

#### The Concept: Horizontal Segments

The body area becomes a **multi-segment horizontal layout**, where each segment represents one level of the current path. Think of it like ranger's miller columns, but leaner — not three equal panels, just enough context to feel spatially oriented.

```
  PARENT (dim)         CURRENT (active)         CHILD PREVIEW (dim)
+----------------+---------------------------+-------------------+
| projects/      |  ▶ src/                   | main.rs           |
|   docs/        |    target/                | app.rs            |
|   scripts/     |    Cargo.toml             | nav.rs            |
|   .git/        |    README.md              | marks.rs          |
|                |    DESIGN_PRD.md          | config.rs         |
|                |                           | palette.rs        |
+----------------+---------------------------+-------------------+
```

**How it works:**
- The **center segment** is the active directory — full brightness, interactive, accepts all keybindings.
- The **left segment** shows the parent directory. The entry corresponding to the current directory is highlighted (cursor memory). This is the "where you came from" context.
- The **right segment** (optional) shows a preview of the hovered entry's children, if it's a directory. This is the "where you could go" context.
- When you press `h` (go parent), the center becomes the right, the left becomes the center, and a new parent loads on the left. The cursor in the new center lands on the directory you just backed out of — **not at the top.**
- When you press `l` (enter), the center becomes the left, the right becomes the center, and a new child preview loads on the right.
- **Jump keys work across all visible segments** — you can jump sideways into a sibling directory in the parent column without navigating back first.

#### Segment Rendering Rules

| Segment | Width | Brightness | Interactive |
|---|---|---|---|
| Left (parent) | ~20% or fixed 18-22 cols | `TEXT_DIM` / `BORDER_MID` | Jump keys only |
| Center (active) | flex (remaining) | Full brightness (normal rendering) | All keybindings |
| Right (child preview) | ~20% or fixed 18-22 cols | `TEXT_DIM` / `BORDER_MID` | None (read-only) |

- Segments are separated by a vertical `BORDER_DIM` line
- At narrow terminals (< 100 cols), collapse to center-only (current behavior)
- At medium terminals (100-139 cols), show left + center (no right preview)
- At wide terminals (140+ cols), show all three segments
- Left and right segments show names only — no type badge, no size, no sigils (just names, dimmed, dense)

#### Cursor Memory

**The fix is simple but critical:** When navigating to a parent directory, find the child directory name in the new entry list and place the cursor there instead of index 0.

Implementation: `navigate_parent()` should save the current directory's name, call `navigate_to(parent)`, then scan `entries` for the matching name and set `cursor` to that index.

This also applies to `navigate_back()` and `navigate_forward()` — any navigation that returns to a previously-visited directory should try to restore the cursor to the last-known position. A `HashMap<PathBuf, String>` (directory -> last highlighted entry name) in app state would handle this cleanly.

#### Jump Across Segments

The two-key jump system extends to the left segment. When Space is pressed:
- Center segment entries get jump keys as usual (homerow first)
- Left segment entries get jump keys from a secondary pool (remaining letters)
- Right segment entries don't get jump keys (just enter the hovered dir first)
- Jumping to a left-segment entry navigates directly there — same as pressing `h` then moving cursor, but instant

This lets you "hop branches" — you're in `src/`, you see `docs/` in the parent column, you jump to it in two keystrokes without backtracking. most importantly, the cursor stays in the center column, it never leaves. the left segment becomes the center segment, the right segment still previews what you are hovering over. the left segment becomes the cwd parent directory. 

#### What This Is NOT

- Not a tree view. There's no expand/collapse, no indentation hierarchy. Each segment is a flat list.
- Not three equal panels. The center dominates. Side segments are context, not co-equal panes.
- Not tabs or splits. There's one focus point (the center), always. The segments are just the frontier of your path rendered visually.

### 13.2 Next: Braille Tree Minimap Widget

A small, dense, purely decorative widget in the lower-right corner of the sidebar showing a braille-rendered tree of the current path context. This is a "you are here" indicator — it doesn't interact, it just looks sick.

```
  PATH MAP
  ⡇⠸⡀
  ⡇ ⠑⡄
  ⡇  ⠈⡆ ◀
  ⡇  ⠈⡇
  ⡇ ⠔⠁
  ⡇⠔⠁
```

**Design:**
- Uses braille Unicode characters (U+2800 block) — each character is a 2x4 dot grid, giving very high density in a small space
- Renders the current path as a vertical spine with branches forking off at each directory level
- The current location is marked with a cursor sigil (from symbol set)
- The tree only shows the current path's lineage (ancestors + siblings at each level), not the entire filesystem
- Max size: ~8 cols x ~10 rows, bottom-right of sidebar
- Colors: spine in `BORDER_MID`, current branch in `TEXT_HOT`, other branches in `BORDER_DIM`
- Only renders when sidebar is visible (>= 100 cols) and there's vertical space remaining below the bookmarks section

**Why braille:**
- 2x4 dot grids mean a 10-row widget can represent 40 levels of depth
- The aesthetic is perfect — it looks like a CRT radar scope or signal trace
- It's pure information density, zero decoration, total corporate utility vibe
- It conforms to the existing symbol philosophy (Unicode glyphs, no emoji, no color-only encoding)

### 13.3 Next: Permission Symbols

Extend the symbol set system to encode file permissions as compact visual glyphs rendered inline in the file list. See Section 4.3.

### 13.4 Future Considerations

- Frecency-based sorting (most-used directories surface higher)
- Configurable keybindings via config file
- Additional symbol sets contributed by users
- Terminal-adaptive symbol set auto-detection

---

*Document version: 2.1 — Added roadmap: fluid tree navigation (miller segments, cursor memory, cross-segment jumping), braille tree minimap, permission symbols.*
