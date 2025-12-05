use crate::config::{Config, Provider};
use crate::constants::*;
use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
    Frame, Terminal,
};
use std::io;

enum InputMode {
    Normal,
    AddProvider(AddProviderState),
    DeleteConfirm,
}

#[derive(Default)]
struct AddProviderState {
    current_field: usize, // 0: name, 1: url, 2: token, 3: model (optional)
    name: String,
    url: String,
    token: String,
    model: String,
}

pub struct TuiApp {
    config: Config,
    list_state: ListState,
    input_mode: InputMode,
    message: Option<String>,
    message_is_error: bool,
}

impl TuiApp {
    pub fn new(config: Config) -> Self {
        let mut list_state = ListState::default();
        if !config.providers.is_empty() {
            list_state.select(Some(0));
        }

        Self {
            config,
            list_state,
            input_mode: InputMode::Normal,
            message: None,
            message_is_error: false,
        }
    }

    fn next(&mut self) {
        if self.config.providers.is_empty() {
            return;
        }
        let i = match self.list_state.selected() {
            Some(i) => {
                if i >= self.config.providers.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    fn previous(&mut self) {
        if self.config.providers.is_empty() {
            return;
        }
        let i = match self.list_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.config.providers.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    fn get_selected_provider(&self) -> Option<&Provider> {
        if let Some(selected) = self.list_state.selected() {
            self.config.providers.values().nth(selected)
        } else {
            None
        }
    }

    fn use_provider(&mut self) -> Result<()> {
        if let Some(provider) = self.get_selected_provider() {
            let name = provider.name.clone();
            let token = provider.token.clone();
            let api_url = provider.api_url.clone();
            let model = provider.model.clone();

            self.config.set_current_provider(&name);
            self.config.save()?;

            // Set environment variables
            let provider = Provider {
                name: name.clone(),
                api_url: api_url.clone(),
                token: token.clone(),
                model: model.clone(),
            };
            set_provider_env_vars(&provider);

            self.message = Some(format!("Switched to provider '{}'", name));
            self.message_is_error = false;
        }
        Ok(())
    }

    fn clear_provider(&mut self) -> Result<()> {
        self.config.clear_current_provider();
        self.config.save()?;

        clear_all_env_vars();

        self.message = Some("Cleared current provider".to_string());
        self.message_is_error = false;
        Ok(())
    }

    fn delete_provider(&mut self) -> Result<()> {
        if let Some(provider) = self.get_selected_provider() {
            let name = provider.name.clone();
            self.config.remove_provider(&name);
            self.config.save()?;

            // Adjust selection
            if self.config.providers.is_empty() {
                self.list_state.select(None);
            } else if let Some(selected) = self.list_state.selected() {
                if selected >= self.config.providers.len() {
                    self.list_state
                        .select(Some(self.config.providers.len() - 1));
                }
            }

            self.message = Some(format!("Deleted provider '{}'", name));
            self.message_is_error = false;
        }
        Ok(())
    }

    fn save_new_provider(&mut self) -> Result<()> {
        if let InputMode::AddProvider(state) = &self.input_mode {
            if state.name.is_empty() || state.url.is_empty() || state.token.is_empty() {
                self.message = Some("Name, URL, and Token are required".to_string());
                self.message_is_error = true;
                return Ok(());
            }

            let model = if state.model.is_empty() {
                None
            } else {
                Some(state.model.clone())
            };

            self.config.add_provider(
                state.name.clone(),
                state.url.clone(),
                state.token.clone(),
                model,
            );
            self.config.save()?;

            // Select the newly added provider
            let index = self.config.providers.len() - 1;
            self.list_state.select(Some(index));

            self.message = Some(format!("Added provider '{}'", state.name));
            self.message_is_error = false;
        }

        self.input_mode = InputMode::Normal;
        Ok(())
    }

    fn handle_input(&mut self, key: KeyCode, modifiers: KeyModifiers) -> Result<bool> {
        match &mut self.input_mode {
            InputMode::Normal => match key {
                KeyCode::Char('q') | KeyCode::Esc => return Ok(true),
                KeyCode::Char('c') if modifiers.contains(KeyModifiers::CONTROL) => return Ok(true),
                KeyCode::Down | KeyCode::Char('j') => self.next(),
                KeyCode::Up | KeyCode::Char('k') => self.previous(),
                KeyCode::Enter | KeyCode::Char('u') => self.use_provider()?,
                KeyCode::Char('a') => {
                    self.input_mode = InputMode::AddProvider(AddProviderState::default());
                    self.message = None;
                }
                KeyCode::Char('d') => {
                    if self.get_selected_provider().is_some() {
                        self.input_mode = InputMode::DeleteConfirm;
                        self.message = None;
                    }
                }
                KeyCode::Char('c') => self.clear_provider()?,
                _ => {}
            },
            InputMode::AddProvider(state) => match key {
                KeyCode::Esc => {
                    self.input_mode = InputMode::Normal;
                    self.message = None;
                }
                KeyCode::Tab => {
                    state.current_field = (state.current_field + 1) % 4;
                }
                KeyCode::BackTab => {
                    state.current_field = if state.current_field == 0 {
                        3
                    } else {
                        state.current_field - 1
                    };
                }
                KeyCode::Enter => {
                    if state.current_field == 3 {
                        self.save_new_provider()?;
                    } else {
                        state.current_field += 1;
                    }
                }
                KeyCode::Backspace => match state.current_field {
                    0 => {
                        state.name.pop();
                    }
                    1 => {
                        state.url.pop();
                    }
                    2 => {
                        state.token.pop();
                    }
                    3 => {
                        state.model.pop();
                    }
                    _ => {}
                },
                KeyCode::Char(c) => match state.current_field {
                    0 => state.name.push(c),
                    1 => state.url.push(c),
                    2 => state.token.push(c),
                    3 => state.model.push(c),
                    _ => {}
                },
                _ => {}
            },
            InputMode::DeleteConfirm => match key {
                KeyCode::Char('y') | KeyCode::Char('Y') => {
                    self.delete_provider()?;
                    self.input_mode = InputMode::Normal;
                }
                KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
                    self.input_mode = InputMode::Normal;
                    self.message = None;
                }
                _ => {}
            },
        }

        Ok(false)
    }
}

pub fn run_tui(config: Config) -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app state
    let mut app = TuiApp::new(config);

    // Run app
    let res = run_app(&mut terminal, &mut app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("Error: {:?}", err);
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut TuiApp) -> Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if app.handle_input(key.code, key.modifiers)? {
                return Ok(());
            }
        }
    }
}

fn ui(f: &mut Frame, app: &mut TuiApp) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(10),
            Constraint::Length(3),
            Constraint::Length(3),
        ])
        .split(f.size());

    // Title
    let title = Paragraph::new("CCE - Claude Config Environment")
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    // Main content area
    match &app.input_mode {
        InputMode::Normal => {
            render_provider_list(f, app, chunks[1]);
        }
        InputMode::AddProvider(state) => {
            render_add_provider_form(f, state, chunks[1]);
        }
        InputMode::DeleteConfirm => {
            render_provider_list(f, app, chunks[1]);
            render_delete_confirmation(f, app, chunks[1]);
        }
    }

    // Status message
    if let Some(ref message) = app.message {
        let style = if app.message_is_error {
            Style::default().fg(Color::Red)
        } else {
            Style::default().fg(Color::Green)
        };
        let msg = Paragraph::new(message.as_str())
            .style(style)
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL).title("Status"));
        f.render_widget(msg, chunks[2]);
    } else {
        let msg = Paragraph::new("").block(Block::default().borders(Borders::ALL).title("Status"));
        f.render_widget(msg, chunks[2]);
    }

    // Help text
    let help_text = match &app.input_mode {
        InputMode::Normal => {
            "↑/↓: Navigate | Enter/u: Use Provider | a: Add | d: Delete | c: Clear | q/Esc: Quit"
        }
        InputMode::AddProvider(_) => "Tab/Shift+Tab: Next/Prev Field | Enter: Save | Esc: Cancel",
        InputMode::DeleteConfirm => "y: Confirm Delete | n/Esc: Cancel",
    };
    let help = Paragraph::new(help_text)
        .style(Style::default().fg(Color::Yellow))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).title("Help"));
    f.render_widget(help, chunks[3]);
}

fn render_provider_list(f: &mut Frame, app: &mut TuiApp, area: Rect) {
    let items: Vec<ListItem> = app
        .config
        .providers
        .values()
        .map(|provider| {
            let is_current = app.config.current_provider.as_ref() == Some(&provider.name);
            let marker = if is_current { "● " } else { "○ " };

            let mut lines = vec![
                Line::from(vec![
                    Span::styled(
                        marker,
                        Style::default().fg(if is_current {
                            Color::Green
                        } else {
                            Color::White
                        }),
                    ),
                    Span::styled(
                        &provider.name,
                        Style::default()
                            .fg(Color::Cyan)
                            .add_modifier(Modifier::BOLD),
                    ),
                ]),
                Line::from(vec![
                    Span::raw("  URL: "),
                    Span::styled(&provider.api_url, Style::default().fg(Color::Yellow)),
                ]),
            ];

            let masked_token = if provider.token.len() > 8 {
                format!("{}****", &provider.token[..8])
            } else {
                "****".to_string()
            };
            lines.push(Line::from(vec![
                Span::raw("  Token: "),
                Span::styled(masked_token, Style::default().fg(Color::DarkGray)),
            ]));

            if let Some(ref model) = provider.model {
                lines.push(Line::from(vec![
                    Span::raw("  Model: "),
                    Span::styled(model, Style::default().fg(Color::Magenta)),
                ]));
            }

            if is_current {
                lines.push(Line::from(Span::styled(
                    "  (currently active)",
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::ITALIC),
                )));
            }

            ListItem::new(lines).style(Style::default())
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Service Providers"),
        )
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    f.render_stateful_widget(list, area, &mut app.list_state);
}

fn render_add_provider_form(f: &mut Frame, state: &AddProviderState, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title("Add New Provider");

    let inner = block.inner(area);
    f.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(1),
        ])
        .split(inner);

    let fields = [
        ("Name", &state.name, 0),
        ("API URL", &state.url, 1),
        ("Token", &state.token, 2),
        ("Model (optional)", &state.model, 3),
    ];

    for (i, (label, value, field_idx)) in fields.iter().enumerate() {
        let is_active = state.current_field == *field_idx;
        let style = if is_active {
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default()
        };

        let border_style = if is_active {
            Style::default().fg(Color::Cyan)
        } else {
            Style::default()
        };

        let input = Paragraph::new(value.as_str()).style(style).block(
            Block::default()
                .borders(Borders::ALL)
                .title(*label)
                .border_style(border_style),
        );
        f.render_widget(input, chunks[i]);

        // Show cursor
        if is_active {
            f.set_cursor(chunks[i].x + value.len() as u16 + 1, chunks[i].y + 1);
        }
    }
}

fn render_delete_confirmation(f: &mut Frame, app: &TuiApp, area: Rect) {
    if let Some(provider) = app.get_selected_provider() {
        let block = Block::default()
            .borders(Borders::ALL)
            .title("Confirm Delete")
            .style(Style::default().bg(Color::Red).fg(Color::White));

        let text = format!(
            "Are you sure you want to delete provider '{}'?\n\nPress 'y' to confirm, 'n' to cancel",
            provider.name
        );

        let paragraph = Paragraph::new(text)
            .block(block)
            .wrap(Wrap { trim: true })
            .alignment(Alignment::Center);

        let popup_area = centered_rect(60, 30, area);
        f.render_widget(paragraph, popup_area);
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
