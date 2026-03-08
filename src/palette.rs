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

/// Create a dimmed version of a palette for side segments.
/// Multiplier < 1.0 darkens, > 1.0 brightens.
impl Palette {
    pub fn dimmed(&self, factor: f32) -> Palette {
        Palette {
            bg: self.bg,
            surface: self.surface,
            text_dim: dim_color(self.text_dim, factor),
            text_mid: dim_color(self.text_mid, factor),
            text_hot: dim_color(self.text_hot, factor),
            border_dim: dim_color(self.border_dim, factor),
            border_mid: dim_color(self.border_mid, factor),
            border_hot: dim_color(self.border_hot, factor),
            warn: self.warn,
        }
    }
}

fn dim_color(color: Color, factor: f32) -> Color {
    match color {
        Color::Rgb(r, g, b) => Color::Rgb(
            ((r as f32) * factor).min(255.0) as u8,
            ((g as f32) * factor).min(255.0) as u8,
            ((b as f32) * factor).min(255.0) as u8,
        ),
        other => other,
    }
}

pub const PALETTE_NAMES: &[&str] = &["phosphor", "amber", "cyan", "red", "pink"];

/// ASCII art corpo logos per palette theme.
/// All logos are exactly 19 chars wide per line.

pub fn corpo_logo(palette_name: &str) -> &'static [&'static str] {
    match palette_name {
        // Green: Factory floor terminal. Assembly line interface division.
        "phosphor" => &[
            "  ╶━━━━━━━━━━━━━━━━━╸  ",
            "  ┃ ▗▄▄▖ STRAHL      ┃  ",
            "  ┃ ▐══▌ ERGONOMIK   ┃  ",
            "  ┃ ▝▀▀▘ ═══════════ ┃  ",
            "  ┃                   ┃  ",
            "  ┃  SHOPFLOOR/OS     ┃  ",
            "  ┃  R.714 ◆ DE-FR-9 ┃  ",
            "  ┃  ░░░░░░░░░▓▓▓███ ┃  ",
            "  ╶━━━━━━━━━━━━━━━━━╸  ",
        ],
        // Amber: Executive briefing console. Strategy & logistics.
        "amber" => &[
            "  ┏╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍┓  ",
            "  ╏  ◈ STRAHL        ╏  ",
            "  ╏    ERGONOMIK     ╏  ",
            "  ╏─────────────────╏  ",
            "  ╏  DIRECTIVE/NET   ╏  ",
            "  ╏  EXEC CONSOLE    ╏  ",
            "  ╏  ▸ AUTH TIER 3   ╏  ",
            "  ╏  ▸ REGION: APAC  ╏  ",
            "  ┗╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍┛  ",
        ],
        // Cyan: Field survey unit. Remote telemetry & diagnostics.
        "cyan" => &[
            "  ┌──┤ STRAHL ├──────┐  ",
            "  │   ERGONOMIK      │  ",
            "  │ ┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄ │  ",
            "  │  FIELD/DIAG v3.1 │  ",
            "  │  ◁ MESH: 14node  │  ",
            "  │  ◁ PING: 0.4ms   │  ",
            "  │  ◁ DRIFT: nominal│  ",
            "  │  ▵ UPLINK ━━ OK  │  ",
            "  └──────────────────┘  ",
        ],
        // Red: Emergency override. Incident response terminal.
        "red" => &[
            "  ▞▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▚  ",
            "  ▌  ╲╱ STRAHL      ▐  ",
            "  ▌     ERGONOMIK   ▐  ",
            "  ▌▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▐  ",
            "  ▌  FAILSAFE/SHELL ▐  ",
            "  ▌  ◤ LOCKOUT MODE ▐  ",
            "  ▌  ◤528Hz ALERT   ▐  ",
            "  ▌  ███░███░███░██ ▐  ",
            "  ▚▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▞  ",
        ],
        // Pink: Neural calibration lab. Experimental cognition mapping.
        "pink" => &[
            "  ·  ˚  ·  ˚  ·  ˚     ",
            "  ╭┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈╮  ",
            "  ┊  ⟐ STRAHL       ┊  ",
            "  ┊    ERGONOMIK    ┊  ",
            "  ┊┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┊  ",
            "  ┊  COGNITION/MAP  ┊  ",
            "  ┊  λ-SYNC  v0.91 ┊  ",
            "  ┊  ∿∿∿∿∿∿∿∿∿∿∿∿  ┊  ",
            "  ╰┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈╯  ",
            "  ˚  ·  ˚  ·  ˚  ·     ",
        ],
        _ => &[
            "  ┌──────────────────┐  ",
            "  │ STRAHL ERGONOMIK │  ",
            "  │ UNREGISTERED TRM │  ",
            "  │ ▸ NO LICENSE     │  ",
            "  └──────────────────┘  ",
        ],
    }
}
