#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct SymbolSet {
    pub name: &'static str,
    pub dir_sigil: &'static str,
    pub file_sigil: &'static str,
    pub cursor: &'static str,
    pub blink_char: &'static str,
    pub depth_up: &'static str,
    pub depth_down: &'static str,
    pub separator: &'static str,
    pub em_dash: &'static str,
    pub warning: &'static str,
    pub bookmark_icon: &'static str,
    pub scrollbar_thumb: &'static str,
    pub scrollbar_track: &'static str,
    pub horizontal_rule: &'static str,
    pub ellipsis: &'static str,
    pub nav_arrows: &'static str,
}

impl SymbolSet {
    pub fn standard() -> Self {
        Self {
            name: "standard",
            dir_sigil: "\u{25a3}",       // ▣
            file_sigil: "\u{25fb}",      // ◻
            cursor: "\u{25b6}",          // ▶
            blink_char: "\u{258b}",      // ▋
            depth_up: "\u{2191}",        // ↑
            depth_down: "\u{2193}",      // ↓
            separator: "\u{00b7}",       // ·
            em_dash: "\u{2014}",         // —
            warning: "\u{26a0}",         // ⚠
            bookmark_icon: "\u{2691}",   // ⚑
            scrollbar_thumb: "\u{2588}", // █
            scrollbar_track: "\u{2502}", // │
            horizontal_rule: "\u{2500}", // ─
            ellipsis: "\u{2026}",        // …
            nav_arrows: "\u{2191}\u{2193}", // ↑↓
        }
    }

    pub fn ascii() -> Self {
        Self {
            name: "ascii",
            dir_sigil: "#",
            file_sigil: "-",
            cursor: ">",
            blink_char: "_",
            depth_up: "^",
            depth_down: "v",
            separator: ".",
            em_dash: "--",
            warning: "!",
            bookmark_icon: "*",
            scrollbar_thumb: "#",
            scrollbar_track: "|",
            horizontal_rule: "-",
            ellipsis: "..",
            nav_arrows: "^v",
        }
    }

    pub fn block() -> Self {
        Self {
            name: "block",
            dir_sigil: "\u{25a0}",       // ■
            file_sigil: "\u{25a1}",      // □
            cursor: "\u{25ba}",          // ►
            blink_char: "\u{2588}",      // █
            depth_up: "\u{25b4}",        // ▴
            depth_down: "\u{25be}",      // ▾
            separator: "\u{2022}",       // •
            em_dash: "\u{2501}",         // ━
            warning: "\u{25c6}",         // ◆
            bookmark_icon: "\u{25c8}",   // ◈
            scrollbar_thumb: "\u{2588}", // █
            scrollbar_track: "\u{2591}", // ░
            horizontal_rule: "\u{2501}", // ━
            ellipsis: "\u{2026}",        // …
            nav_arrows: "\u{25b4}\u{25be}", // ▴▾
        }
    }

    pub fn minimal() -> Self {
        Self {
            name: "minimal",
            dir_sigil: "\u{25cf}",       // ●
            file_sigil: "\u{25cb}",      // ○
            cursor: "\u{25b8}",          // ▸
            blink_char: "\u{2502}",      // │
            depth_up: "\u{2039}",        // ‹
            depth_down: "\u{203a}",      // ›
            separator: "\u{2027}",       // ‧
            em_dash: "\u{2013}",         // –
            warning: "\u{00d7}",         // ×
            bookmark_icon: "\u{2020}",   // †
            scrollbar_thumb: "\u{2503}", // ┃
            scrollbar_track: "\u{2506}", // ┆
            horizontal_rule: "\u{2508}", // ┈
            ellipsis: "\u{2026}",        // …
            nav_arrows: "\u{25b8}\u{25be}", // ▸▾
        }
    }

    pub fn pipeline() -> Self {
        Self {
            name: "pipeline",
            dir_sigil: "\u{25c9}",       // ◉
            file_sigil: "\u{25ce}",      // ◎
            cursor: "\u{2192}",          // →
            blink_char: "\u{258f}",      // ▏
            depth_up: "\u{2190}",        // ←
            depth_down: "\u{2192}",      // →
            separator: "\u{2502}",       // │
            em_dash: "\u{2500}",         // ─
            warning: "\u{2297}",         // ⊗
            bookmark_icon: "\u{2302}",   // ⌂
            scrollbar_thumb: "\u{2503}", // ┃
            scrollbar_track: "\u{250a}", // ┊
            horizontal_rule: "\u{2500}", // ─
            ellipsis: "\u{2026}",        // …
            nav_arrows: "\u{2190}\u{2192}", // ←→
        }
    }

    pub fn from_name(name: &str) -> Self {
        match name {
            "ascii" => Self::ascii(),
            "block" => Self::block(),
            "minimal" => Self::minimal(),
            "pipeline" => Self::pipeline(),
            _ => Self::standard(),
        }
    }
}

pub const SYMBOL_SET_NAMES: &[&str] = &["standard", "ascii", "block", "minimal", "pipeline"];
