#![allow(dead_code)]

use std::{env, fs, path::PathBuf};

use anyhow::{bail, Context, Result};
use crossterm::event::{Event, KeyCode, MouseEventKind};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use tree::ContentType;

use crate::interactive::tree_overview::TreeOverview;
use crate::{config::Config, tree::Tree};

mod config;
mod interactive;
mod tree;

fn main() -> Result<()> {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        bail!("invalid usage");
    }

    let path = args.get(1).unwrap();
    let content_type = if path.ends_with(".yaml") {
        Some(ContentType::Yaml)
    } else if path.ends_with(".toml") {
        Some(ContentType::Toml)
    } else if path.ends_with(".json") {
        Some(ContentType::Json)
    } else {
        None
    };
    let path = PathBuf::from(path);

    let data = fs::read(path).context("read file")?;
    let data = String::from_utf8(data).context("parse utf8")?;

    let mut cfg = Config::default();
    cfg.validate().context("validate config")?;

    let tree = Tree::parse(&cfg, &data, content_type)?;
    let widget = TreeOverview::new(&cfg, tree);
    draw(widget)?;

    Ok(())
}

fn draw(mut widget: TreeOverview) -> Result<()> {
    // Terminal initialization
    crossterm::terminal::enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    crossterm::execute!(
        stdout,
        crossterm::terminal::EnterAlternateScreen,
        crossterm::event::EnableMouseCapture
    )?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout))?;

    terminal.draw(|frame| widget.draw(frame, frame.size()))?;
    loop {
        let update = match crossterm::event::read()? {
            Event::Key(key) => match key.code {
                KeyCode::Char('q') => {
                    // restore terminal
                    crossterm::terminal::disable_raw_mode()?;
                    crossterm::execute!(
                        terminal.backend_mut(),
                        crossterm::terminal::LeaveAlternateScreen,
                        crossterm::event::DisableMouseCapture
                    )?;
                    terminal.show_cursor()?;
                    return Ok(());
                }
                KeyCode::Char('\n' | ' ') => widget.state.toggle_selected(),
                KeyCode::Left => widget.state.key_left(),
                KeyCode::Right => widget.state.key_right(),
                KeyCode::Down => widget.state.key_down(&widget.tree.items),
                KeyCode::Up => widget.state.key_up(&widget.tree.items),
                KeyCode::Esc => widget.state.select(Vec::new()),
                KeyCode::Home => widget.state.select_first(&widget.tree.items),
                KeyCode::End => widget.state.select_last(&widget.tree.items),
                KeyCode::PageDown => widget.state.scroll_down(3),
                KeyCode::PageUp => widget.state.scroll_up(3),
                _ => false,
            },
            Event::Mouse(mouse) => match mouse.kind {
                MouseEventKind::ScrollDown => widget.state.scroll_down(1),
                MouseEventKind::ScrollUp => widget.state.scroll_up(1),
                _ => false,
            },
            _ => false,
        };
        if update {
            terminal.draw(|frame| widget.draw(frame, frame.size()))?;
        }
    }
}
