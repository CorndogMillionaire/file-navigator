# REM — Remote Entry Module

A minimal, retro sci-fi file navigator for the terminal. Built with Rust, `ratatui`, and `crossterm`.

REM does one thing: navigate a file tree and emit a path. The aesthetic is inspired by late-70s/early-80s sci-fi — phosphor CRTs, corporate bureaucracy, degraded signal. Think *Alien* (1979).

```
 REM NAVIGATOR  ·  FILE SYSTEM              ITEMS:1847  ·  THEME:PHOSPHOR  ·  SYS:NOMINAL
 /home / projects / rem / ▋
 [a]  ▣  src/                                  DIR      —
 [s]  ▣  target/                               DIR      —
 [d]  ◻  Cargo.toml                            TOML  1.2 KB
 [f]  ◻  README.md                             MD    2.4 KB
 hjkl move  ·  a–z jump  ·  / fuzzy  ·  Space jump keys  ·  q quit
```

## Install

Requires Rust 1.85+ (edition 2024).

```bash
git clone https://github.com/YOUR_USERNAME/file-navigator.git
cd file-navigator
cargo install --path .
```

This installs the `rem` binary to `~/.cargo/bin/`.

## Shell Integration

REM outputs a selected directory path to stdout, but a subprocess can't change your shell's working directory. You need a small wrapper function so that `rem` acts like `cd`.

### Automatic setup

```bash
rem --shell-init >> ~/.bashrc   # or ~/.zshrc
source ~/.bashrc
```

This adds a `rem()` shell function that captures REM's output and `cd`s into it.

### Manual setup

Add this to your `~/.bashrc` or `~/.zshrc`:

```bash
rem() {
  local result
  result=$(command rem "$@")
  if [ $? -eq 0 ] && [ -n "$result" ]; then
    cd "$result" || return
  fi
}
```

Then reload your shell:

```bash
source ~/.bashrc  # or ~/.zshrc
```

Now running `rem` opens the navigator, and selecting a directory `cd`s you into it.

## Usage

```
rem [OPTIONS]
```

| Option | Description |
|---|---|
| `--palette <NAME>` | Color palette: `phosphor` (default), `amber`, `cyan`, `red`, `pink` |
| `--shell-init` | Print the shell wrapper function |
| `--help` | Show help |

The palette can also be set permanently in `~/.config/rem/config.toml`:

```toml
palette = "amber"
```

## Keybindings

| Key | Action |
|---|---|
| `h` / Left | Parent directory |
| `l` / Right / Enter | Enter directory |
| `j` / Down | Cursor down |
| `k` / Up | Cursor up |
| `s` | Select current directory (emit path + exit) |
| `/` | Fuzzy search |
| `Space` | Jump key overlay |
| `b` | Open bookmarks |
| `B` | Bookmark current directory |
| `t` | Cycle color theme |
| `.` | Toggle hidden files |
| `q` / Esc | Quit |

## Color Themes

- **Phosphor Green** — The mainframe. Institutional. Cold.
- **Amber Corporate** — The executive terminal. Every access is logged.
- **Degraded Cyan** — The field unit. Dropped one too many times.
- **Red** — Emergency lighting.
- **Pink** — Corrupted signal.

Cycle through themes with `t` during navigation.

## License

GPL3
https://www.gnu.org/licenses/gpl-3.0.en.html
