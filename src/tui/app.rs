//! TUI Application - Main application state and event handling

use crate::tui::TuiConfig;
use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, Borders, List, ListItem, Paragraph, Tabs},
    Terminal,
};
use std::io;

pub struct TuiApp {
    terminal: Terminal<CrosstermBackend<io::Stdout>>,
    #[allow(dead_code)]
    config: TuiConfig,
    messages: Vec<String>,
    input: String,
    history: Vec<String>,
    history_index: usize,
    current_tab: usize,
    tabs: Vec<String>,
}

impl TuiApp {
    pub fn new(config: TuiConfig) -> anyhow::Result<Self> {
        let backend = CrosstermBackend::new(io::stdout());
        let terminal = Terminal::new(backend)?;

        Ok(Self {
            terminal,
            config,
            messages: vec![],
            input: String::new(),
            history: vec![],
            history_index: 0,
            current_tab: 0,
            tabs: vec![
                "Chat".to_string(),
                "Sessions".to_string(),
                "Memory".to_string(),
                "Settings".to_string(),
            ],
        })
    }

    pub fn run(&mut self) -> anyhow::Result<()> {
        enable_raw_mode()?;
        self.terminal.clear()?;

        let res = self.run_loop();

        disable_raw_mode()?;
        execute!(self.terminal.backend_mut(), LeaveAlternateScreen,)?;
        self.terminal.show_cursor()?;

        res
    }

    fn run_loop(&mut self) -> anyhow::Result<()> {
        self.messages
            .push("🦀 krabkrab TUI - Type /help for commands".to_string());
        self.messages.push("─".repeat(50));

        loop {
            self.draw()?;

            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL) {
                    break;
                }
                self.handle_key_event(key);
            }

            if self.should_quit() {
                break;
            }
        }

        Ok(())
    }

    fn draw(&mut self) -> anyhow::Result<()> {
        let messages = self.messages.clone();
        let input = self.input.clone();
        let current_tab = self.current_tab;
        let tabs = self.tabs.clone();

        self.terminal.draw(|f| {
            let area = f.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Min(0),
                    Constraint::Length(3),
                ])
                .split(area);

            // Header
            let titles: Vec<Line> = tabs.iter().map(|t| Line::from(t.as_str())).collect();
            let tabs_widget = Tabs::new(titles)
                .select(current_tab)
                .style(Style::default().fg(Color::Cyan))
                .highlight_style(
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                )
                .block(Block::default().borders(Borders::ALL).title("🦀 krabkrab"));
            f.render_widget(tabs_widget, chunks[0]);

            // Content
            let content: Vec<ListItem> = messages
                .iter()
                .rev()
                .take(chunks[1].height as usize)
                .map(|m| ListItem::new(m.as_str()))
                .collect();
            let list = List::new(content)
                .block(Block::default().borders(Borders::ALL).title("Chat"))
                .style(Style::default().fg(Color::White));
            f.render_widget(list, chunks[1]);

            // Input
            let input_widget = Paragraph::new(input.as_str())
                .style(Style::default().fg(Color::Green))
                .block(Block::default().borders(Borders::ALL).title("Input"));
            f.render_widget(input_widget, chunks[2]);
        })?;
        Ok(())
    }

    fn should_quit(&self) -> bool {
        self.input == "quit" || self.input == "exit" || self.input == "q"
    }

    fn handle_key_event(&mut self, key: crossterm::event::KeyEvent) {
        match key.code {
            KeyCode::Char(c) => {
                self.input.push(c);
            }
            KeyCode::Backspace => {
                self.input.pop();
            }
            KeyCode::Enter => {
                self.handle_submit();
            }
            KeyCode::Tab => {
                self.current_tab = (self.current_tab + 1) % self.tabs.len();
            }
            KeyCode::Up => {
                if !self.history.is_empty() && self.history_index > 0 {
                    self.history_index -= 1;
                    self.input = self.history[self.history_index].clone();
                }
            }
            KeyCode::Down => {
                if !self.history.is_empty() && self.history_index < self.history.len() - 1 {
                    self.history_index += 1;
                    self.input = self.history[self.history_index].clone();
                } else {
                    self.history_index = self.history.len();
                    self.input.clear();
                }
            }
            KeyCode::Esc => {
                self.input.clear();
            }
            _ => {}
        }
    }

    fn handle_submit(&mut self) {
        let text = self.input.trim().to_string();
        if text.is_empty() {
            return;
        }

        if !self.history.contains(&text) {
            self.history.push(text.clone());
        }
        self.history_index = self.history.len();
        self.input.clear();

        if text.starts_with('/') {
            self.handle_command(&text);
        } else {
            self.messages.push(format!("\x1b[36m> {}\x1b[0m", text));
        }
    }

    fn handle_command(&mut self, cmd: &str) {
        match cmd.to_lowercase().as_str() {
            "/help" | "/h" => {
                self.messages.push("Commands:".to_string());
                self.messages
                    .push("  /help     - Show this help".to_string());
                self.messages.push("  /clear    - Clear chat".to_string());
                self.messages
                    .push("  /sessions - List sessions".to_string());
                self.messages
                    .push("  /memory   - Search memory".to_string());
                self.messages.push("  /quit     - Exit".to_string());
            }
            "/clear" | "/c" => {
                self.messages.clear();
                self.messages.push("Chat cleared".to_string());
            }
            "/sessions" => {
                self.messages.push("Sessions:".to_string());
                self.messages.push("  main - Main chat session".to_string());
            }
            "/memory" => {
                self.messages
                    .push("Use: krabkrab memory search <query>".to_string());
            }
            "/quit" | "/exit" | "/q" => {
                self.messages.push("Goodbye! 🦀".to_string());
            }
            _ => {
                self.messages.push(format!("Unknown command: {}", cmd));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tui_config_defaults() {
        let config = TuiConfig::default();
        assert_eq!(config.gateway_url, "http://localhost:18789");
        assert_eq!(config.session, "main");
    }
}
