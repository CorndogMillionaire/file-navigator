mod app;
mod config;
mod marks;
mod nav;
mod palette;
mod ui;

use std::io;
use std::time::Duration;

use crossterm::event::{self, Event};
use crossterm::execute;
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;

use app::App;
use palette::Palette;

fn print_help() {
    eprintln!(
        r#"REM NAVIGATOR — Remote Entry Module
A retro sci-fi file navigator TUI.

USAGE:
    rem [OPTIONS]

OPTIONS:
    --palette <NAME>    Set color palette: phosphor, amber, cyan, red, pink
    --help              Show this help message
    --shell-init        Print shell integration function

SHELL INTEGRATION:
    rem outputs the selected directory to stdout so your shell can cd into it.
    Since a subprocess can't change the parent shell's directory, you need a
    wrapper function. Run: rem --shell-init

KEYS:
    hjkl / arrows   Navigate
    l / Enter       Enter directory
    h / -           Go to parent
    s               Select current dir (emit path + exit)
    /               Fuzzy search (also searches sub/parent dirs)
    Space           Jump keys
    b               Open bookmarks popup
    B               Add current dir as bookmark
    t               Cycle color theme
    .               Toggle hidden files
    q / Esc         Quit
"#
    );
}

fn print_shell_init() {
    // Resolve the actual binary path so the shell function can shadow the name "rem"
    let bin_path = std::env::current_exe()
        .map(|p| p.display().to_string())
        .unwrap_or_else(|_| "rem".to_string());
    println!(
        r#"# Add this to your .bashrc or .zshrc:
rem() {{
  local result
  result=$({bin} "$@")
  if [ $? -eq 0 ] && [ -n "$result" ]; then
    cd "$result" || return
  fi
}}"#,
        bin = bin_path
    );
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Handle --help and --shell-init before terminal setup
    let args: Vec<String> = std::env::args().collect();
    for arg in &args[1..] {
        match arg.as_str() {
            "--help" | "-h" => {
                print_help();
                return Ok(());
            }
            "--shell-init" => {
                print_shell_init();
                return Ok(());
            }
            _ => {}
        }
    }

    // Set up panic hook to restore terminal
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        let _ = disable_raw_mode();
        let _ = execute!(io::stderr(), LeaveAlternateScreen);
        original_hook(panic_info);
    }));

    // Terminal setup on stderr (stdout reserved for path output)
    enable_raw_mode()?;
    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    let palette_name = config::load_palette_name();
    let palette_index = match palette_name.as_str() {
        "amber" => 1,
        "cyan" => 2,
        "red" => 3,
        "pink" => 4,
        _ => 0,
    };
    let palette = Palette::from_name(&palette_name);
    let mut app = App::new(palette, palette_index, None);

    // Main event loop
    loop {
        terminal.draw(|f| ui::render(f, &mut app))?;

        if event::poll(Duration::from_millis(100))? {
            match event::read()? {
                Event::Key(key) => {
                    app.handle_key(key);
                    if let Some(ref path) = app.selected_path {
                        // Emit selected file path to stdout
                        println!("{}", path.display());
                        break;
                    }
                    if app.should_quit {
                        disable_raw_mode()?;
                        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
                        std::process::exit(1);
                    }
                }
                Event::Resize(_, _) => {}
                _ => {}
            }
        }

        app.tick();
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}
