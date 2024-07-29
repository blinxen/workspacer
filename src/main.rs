mod config;
mod terminal;
mod utils;
mod workspaces;

use std::io::{stdout, Write};
use std::process::Command;

use config::Config;
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use crossterm::style::{Color, PrintStyledContent, Stylize};
use crossterm::QueueableCommand;
use workspaces::Workspace;

fn exec_workspace(config: &Config, workspace: &Workspace) -> Result<(), std::io::Error> {
    terminal::restore_terminal()?;
    if let Err(error) = Command::new(&config.command)
        .current_dir(&workspace.path)
        .status()
    {
        eprintln!(
            "Could not execute command {} with workspace {}",
            &config.command, &workspace.path
        );
        eprintln!("{}", error);
    }
    terminal::prepare_terminal()?;

    Ok(())
}

fn render(workspaces: &[Workspace], selected: usize) -> Result<(), std::io::Error> {
    let mut area = terminal::size()?;
    area.width /= 2;
    area.height /= 2;
    area.x = area.width / 2;
    area.y = area.height / 2;

    utils::border(&area, "workspacer")?;
    // Draw workspaces
    utils::reset_cursor_in_area(&area)?;
    for (index, workspace) in workspaces.iter().enumerate() {
        utils::go_to_next_line_in_area(&area, 1)?;
        let mut styled_workspace = utils::build_line(
            &format!("{} <{}>", workspace.title, workspace.path),
            (area.width - 2) as usize,
        )
        .stylize();
        if index == selected {
            styled_workspace = styled_workspace.on(Color::White).with(Color::Black);
        }
        stdout().queue(PrintStyledContent(styled_workspace))?;
    }

    // Make sure everything is drawn before waiting for key event
    stdout().flush()?;

    Ok(())
}

fn main() -> Result<(), std::io::Error> {
    terminal::prepare_terminal()?;
    let config = config::read();
    let workspaces = workspaces::read_workspaces(&config.workspaces);
    let mut selected_workspace = 0;
    loop {
        render(&workspaces, selected_workspace)?;
        if let Event::Key(KeyEvent { code, .. }) = event::read()? {
            match code {
                KeyCode::Char('q') => break,
                KeyCode::Enter => {
                    exec_workspace(&config, workspaces.get(selected_workspace).unwrap())?
                }
                KeyCode::Up => {
                    selected_workspace = if selected_workspace == 0 {
                        workspaces.len() - 1
                    } else {
                        selected_workspace - 1
                    }
                }
                KeyCode::Down => {
                    selected_workspace = if selected_workspace == (workspaces.len() - 1) {
                        0
                    } else {
                        selected_workspace + 1
                    }
                }
                _ => {}
            }
        }
    }
    terminal::restore_terminal()?;

    Ok(())
}
