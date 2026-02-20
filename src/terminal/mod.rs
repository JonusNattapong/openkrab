//! terminal — ANSI terminal utilities: colours, tables, progress, themes.
//! Ported from `openkrab/src/terminal/` (Phase 8).
//!
//! Provides a minimal, zero-dependency terminal formatting layer for
//! the krabkrab-cli output.

// ─── ANSI colours ─────────────────────────────────────────────────────────────

/// ANSI colour code constants.
pub mod ansi {
    pub const RESET: &str = "\x1b[0m";
    pub const BOLD: &str = "\x1b[1m";
    pub const DIM: &str = "\x1b[2m";
    pub const ITALIC: &str = "\x1b[3m";
    pub const UNDERLINE: &str = "\x1b[4m";

    // Foreground colours
    pub const BLACK: &str = "\x1b[30m";
    pub const RED: &str = "\x1b[31m";
    pub const GREEN: &str = "\x1b[32m";
    pub const YELLOW: &str = "\x1b[33m";
    pub const BLUE: &str = "\x1b[34m";
    pub const MAGENTA: &str = "\x1b[35m";
    pub const CYAN: &str = "\x1b[36m";
    pub const WHITE: &str = "\x1b[37m";
    pub const BRIGHT_BLACK: &str = "\x1b[90m";
    pub const BRIGHT_RED: &str = "\x1b[91m";
    pub const BRIGHT_GREEN: &str = "\x1b[92m";
    pub const BRIGHT_YELLOW: &str = "\x1b[93m";
    pub const BRIGHT_BLUE: &str = "\x1b[94m";
    pub const BRIGHT_MAGENTA: &str = "\x1b[95m";
    pub const BRIGHT_CYAN: &str = "\x1b[96m";
    pub const BRIGHT_WHITE: &str = "\x1b[97m";
}

/// Wrap `text` in an ANSI colour code, resetting afterwards.
pub fn color(code: &str, text: &str) -> String {
    format!("{}{}{}", code, text, ansi::RESET)
}

pub fn bold(text: &str) -> String {
    color(ansi::BOLD, text)
}
pub fn dim(text: &str) -> String {
    color(ansi::DIM, text)
}
pub fn red(text: &str) -> String {
    color(ansi::RED, text)
}
pub fn green(text: &str) -> String {
    color(ansi::GREEN, text)
}
pub fn yellow(text: &str) -> String {
    color(ansi::YELLOW, text)
}
pub fn blue(text: &str) -> String {
    color(ansi::BLUE, text)
}
pub fn cyan(text: &str) -> String {
    color(ansi::CYAN, text)
}
pub fn magenta(text: &str) -> String {
    color(ansi::MAGENTA, text)
}

// ─── Theme ────────────────────────────────────────────────────────────────────

/// Semantic colour theme for the CLI.
pub struct Theme {
    pub no_color: bool,
}

impl Theme {
    pub fn new() -> Self {
        let no_color =
            std::env::var("NO_COLOR").is_ok() || std::env::var("TERM").as_deref() == Ok("dumb");
        Self { no_color }
    }

    fn apply(&self, code: &str, text: &str) -> String {
        if self.no_color {
            text.to_string()
        } else {
            color(code, text)
        }
    }

    pub fn success(&self, text: &str) -> String {
        self.apply(ansi::BRIGHT_GREEN, text)
    }
    pub fn error(&self, text: &str) -> String {
        self.apply(ansi::BRIGHT_RED, text)
    }
    pub fn warning(&self, text: &str) -> String {
        self.apply(ansi::BRIGHT_YELLOW, text)
    }
    pub fn info(&self, text: &str) -> String {
        self.apply(ansi::BRIGHT_CYAN, text)
    }
    pub fn muted(&self, text: &str) -> String {
        self.apply(ansi::BRIGHT_BLACK, text)
    }
    pub fn label(&self, text: &str) -> String {
        self.apply(ansi::BOLD, text)
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::new()
    }
}

// ─── Status icons ─────────────────────────────────────────────────────────────

pub fn icon_ok() -> &'static str {
    "✓"
}
pub fn icon_err() -> &'static str {
    "✗"
}
pub fn icon_warn() -> &'static str {
    "⚠"
}
pub fn icon_info() -> &'static str {
    "ℹ"
}
pub fn icon_arrow() -> &'static str {
    "→"
}

// ─── Health style ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HealthStatus {
    Ok,
    Warning,
    Error,
    Unknown,
}

pub fn health_line(theme: &Theme, status: &HealthStatus, label: &str, detail: &str) -> String {
    let (icon, styled) = match status {
        HealthStatus::Ok => (icon_ok(), theme.success(label)),
        HealthStatus::Warning => (icon_warn(), theme.warning(label)),
        HealthStatus::Error => (icon_err(), theme.error(label)),
        HealthStatus::Unknown => (icon_info(), theme.muted(label)),
    };
    format!("{} {}  {}", icon, styled, theme.muted(detail))
}

// ─── Table ────────────────────────────────────────────────────────────────────

/// Simple fixed-width ASCII table for CLI output.
pub struct Table {
    pub headers: Vec<String>,
    pub rows: Vec<Vec<String>>,
}

impl Table {
    pub fn new(headers: Vec<impl Into<String>>) -> Self {
        Self {
            headers: headers.into_iter().map(|h| h.into()).collect(),
            rows: Vec::new(),
        }
    }

    pub fn add_row(&mut self, row: Vec<impl Into<String>>) {
        self.rows.push(row.into_iter().map(|c| c.into()).collect());
    }

    pub fn render(&self) -> String {
        let col_count = self.headers.len();
        let mut widths: Vec<usize> = self.headers.iter().map(|h| h.len()).collect();

        for row in &self.rows {
            for (i, cell) in row.iter().enumerate() {
                if i < col_count {
                    widths[i] = widths[i].max(cell.len());
                }
            }
        }

        let sep = widths
            .iter()
            .map(|&w| "-".repeat(w + 2))
            .collect::<Vec<_>>()
            .join("+");
        let sep = format!("+{}+", sep);

        let format_row = |row: &[String]| -> String {
            let cells: Vec<String> = (0..col_count)
                .map(|i| {
                    let cell = row.get(i).map(|s| s.as_str()).unwrap_or("");
                    format!(" {:<width$} ", cell, width = widths[i])
                })
                .collect();
            format!("|{}|", cells.join("|"))
        };

        let mut out = String::new();
        out.push_str(&sep);
        out.push('\n');
        out.push_str(&format_row(&self.headers));
        out.push('\n');
        out.push_str(&sep);
        out.push('\n');
        for row in &self.rows {
            out.push_str(&format_row(row));
            out.push('\n');
        }
        out.push_str(&sep);
        out
    }
}

// ─── Progress line ────────────────────────────────────────────────────────────

/// A simple single-line ASCII progress bar.
pub struct ProgressLine {
    pub label: String,
    pub width: usize,
    pub fill_char: char,
    pub empty_char: char,
}

impl ProgressLine {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            width: 30,
            fill_char: '█',
            empty_char: '░',
        }
    }

    /// Render at `fraction` (0.0–1.0).
    pub fn render(&self, fraction: f32) -> String {
        let fraction = fraction.clamp(0.0, 1.0);
        let filled = (fraction * self.width as f32).round() as usize;
        let empty = self.width - filled;
        let bar: String = std::iter::repeat(self.fill_char)
            .take(filled)
            .chain(std::iter::repeat(self.empty_char).take(empty))
            .collect();
        format!("{} [{}] {:.0}%", self.label, bar, fraction * 100.0)
    }
}

// ─── Note helper ─────────────────────────────────────────────────────────────

/// Print a formatted note to stdout.
pub fn note(theme: &Theme, kind: &str, text: &str) {
    let prefix = match kind {
        "success" | "ok" => theme.success(&format!("{} ", icon_ok())),
        "error" => theme.error(&format!("{} ", icon_err())),
        "warning" | "warn" => theme.warning(&format!("{} ", icon_warn())),
        _ => theme.info(&format!("{} ", icon_info())),
    };
    println!("{}{}", prefix, text);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color_wraps_text() {
        let s = color(ansi::RED, "hello");
        assert!(s.contains("hello"));
        assert!(s.contains(ansi::RED));
        assert!(s.contains(ansi::RESET));
    }

    #[test]
    fn theme_no_color_passthrough() {
        let t = Theme { no_color: true };
        assert_eq!(t.success("ok"), "ok");
        assert_eq!(t.error("fail"), "fail");
    }

    #[test]
    fn table_render() {
        let mut t = Table::new(vec!["Name", "Status"]);
        t.add_row(vec!["krabkrab", "running"]);
        t.add_row(vec!["memory", "ok"]);
        let rendered = t.render();
        assert!(rendered.contains("krabkrab"));
        assert!(rendered.contains("running"));
        assert!(rendered.contains("Name"));
    }

    #[test]
    fn progress_line_render() {
        let p = ProgressLine::new("Loading");
        let s = p.render(0.5);
        assert!(s.contains("Loading"));
        assert!(s.contains("50%"));
    }

    #[test]
    fn progress_clamps() {
        let p = ProgressLine::new("test");
        let s = p.render(1.5); // should clamp to 1.0
        assert!(s.contains("100%"));
    }

    #[test]
    fn health_line_renders() {
        let t = Theme { no_color: true };
        let s = health_line(&t, &HealthStatus::Ok, "Telegram", "connected");
        assert!(s.contains("Telegram"));
        assert!(s.contains("connected"));
    }
}
