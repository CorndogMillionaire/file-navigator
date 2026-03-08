use ratatui::style::Color;

#[derive(Clone)]
pub struct Palette {
    pub bg: Color,
    pub surface: Color,
    pub text_dim: Color,
    pub text_mid: Color,
    pub text_hot: Color,
    pub border_dim: Color,
    pub border_mid: Color,
    pub border_hot: Color,
    pub warn: Color,
}

impl Palette {
    pub fn phosphor_green() -> Self {
        Self {
            bg: Color::Rgb(3, 3, 3),
            surface: Color::Rgb(2, 12, 2),
            text_dim: Color::Rgb(0, 82, 24),
            text_mid: Color::Rgb(0, 168, 40),
            text_hot: Color::Rgb(0, 255, 65),
            border_dim: Color::Rgb(0, 26, 8),
            border_mid: Color::Rgb(0, 61, 16),
            border_hot: Color::Rgb(0, 122, 34),
            warn: Color::Rgb(255, 68, 68),
        }
    }

    pub fn amber() -> Self {
        Self {
            bg: Color::Rgb(12, 8, 0),
            surface: Color::Rgb(17, 10, 0),
            text_dim: Color::Rgb(90, 58, 0),
            text_mid: Color::Rgb(196, 122, 0),
            text_hot: Color::Rgb(255, 176, 0),
            border_dim: Color::Rgb(58, 40, 0),
            border_mid: Color::Rgb(107, 74, 0),
            border_hot: Color::Rgb(128, 88, 0),
            warn: Color::Rgb(255, 68, 68),
        }
    }

    pub fn degraded_cyan() -> Self {
        Self {
            bg: Color::Rgb(1, 10, 13),
            surface: Color::Rgb(1, 13, 16),
            text_dim: Color::Rgb(0, 96, 112),
            text_mid: Color::Rgb(0, 149, 168),
            text_hot: Color::Rgb(0, 229, 255),
            border_dim: Color::Rgb(0, 21, 32),
            border_mid: Color::Rgb(0, 48, 64),
            border_hot: Color::Rgb(0, 96, 122),
            warn: Color::Rgb(255, 68, 68),
        }
    }

    pub fn crimson_red() -> Self {
        Self {
            bg: Color::Rgb(10, 2, 2),
            surface: Color::Rgb(18, 4, 4),
            text_dim: Color::Rgb(120, 20, 20),
            text_mid: Color::Rgb(200, 40, 40),
            text_hot: Color::Rgb(255, 60, 48),
            border_dim: Color::Rgb(40, 8, 8),
            border_mid: Color::Rgb(80, 16, 16),
            border_hot: Color::Rgb(140, 28, 28),
            warn: Color::Rgb(255, 200, 60),
        }
    }

    pub fn hot_pink() -> Self {
        Self {
            bg: Color::Rgb(8, 2, 10),
            surface: Color::Rgb(14, 4, 16),
            text_dim: Color::Rgb(120, 30, 100),
            text_mid: Color::Rgb(200, 50, 160),
            text_hot: Color::Rgb(255, 80, 200),
            border_dim: Color::Rgb(40, 10, 32),
            border_mid: Color::Rgb(80, 20, 64),
            border_hot: Color::Rgb(140, 36, 112),
            warn: Color::Rgb(255, 255, 100),
        }
    }

    pub fn from_name(name: &str) -> Self {
        match name {
            "amber" => Self::amber(),
            "cyan" => Self::degraded_cyan(),
            "red" => Self::crimson_red(),
            "pink" => Self::hot_pink(),
            _ => Self::phosphor_green(),
        }
    }
}

pub const PALETTE_NAMES: &[&str] = &["phosphor", "amber", "cyan", "red", "pink"];

/// ASCII art corpo logos per palette theme.
/// All logos are exactly 19 chars wide per line.
pub fn corpo_logo(palette_name: &str) -> &'static [&'static str] {
    match palette_name {
        "phosphor" => &[
            " ┌───────────────┐ ",
            " │  TYRELL       │ ",
            " │  SYSTEMS CORP │ ",
            " │ ───────────── │ ",
            " │  MORE HUMAN   │ ",
            " │  THAN HUMAN   │ ",
            " └───────────────┘ ",
        ],
        "amber" => &[
            " ╔═══════════════╗ ",
            " ║ WEYLAND-YUTAN ║ ",
            " ║───────────────║ ",
            " ║  BUILDING     ║ ",
            " ║  BETTER       ║ ",
            " ║  WORLDS       ║ ",
            " ╚═══════════════╝ ",
        ],
        "cyan" => &[
            " ┌──┬─────┬──┐    ",
            " │▓▓│     │▓▓│    ",
            " ├──┘     └──┤    ",
            " │  SEEGSON   │    ",
            " │  SYNTH-7   │    ",
            " │  FIELD OPS │    ",
            " └────────────┘    ",
        ],
        "red" => &[
            " ▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄  ",
            " █ ╳ REDCORP  ╳ █  ",
            " █─────────────█  ",
            " █  HAZARD OPS  █  ",
            " █  CLEARANCE   █  ",
            " █  LEVEL: ░░░  █  ",
            " ▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀  ",
        ],
        "pink" => &[
            "    *  .  *  .     ",
            " ┌──────────────┐  ",
            " │ PARADIGM     │  ",
            " │ NEURAL  LABS │  ",
            " │~~~~~~~~~~~~~~│  ",
            " │ DREAM.INJECT │  ",
            " │ v4.08 //LIVE │  ",
            " └──────────────┘  ",
            "    .  *  .  *     ",
        ],
        _ => &[
            " ┌─────────────┐   ",
            " │ UNKNOWN     │   ",
            " │ UNIT        │   ",
            " └─────────────┘   ",
        ],
    }
}
