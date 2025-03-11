use std::io::{stdout, Write};

use crossterm::event::KeyCode;
use crossterm::style::{Color, PrintStyledContent, Stylize};
use crossterm::QueueableCommand;

use crate::config::Config;
use crate::workspaces::{self, Workspace};
use crate::{terminal, utils};

pub struct App {
    workspaces: Vec<Workspace>,
    config: Config,
    selected_workspace: usize,
    pub quit: bool,
}

impl App {
    pub fn new() -> Self {
        let config = Config::read();
        App {
            workspaces: workspaces::read_workspaces(&config.workspaces),
            config,
            selected_workspace: 0,
            quit: false,
        }
    }

    pub fn render(&self) -> Result<(), std::io::Error> {
        let mut area = terminal::size()?;
        area.width /= 2;
        area.height /= 2;
        area.x = area.width / 2;
        area.y = area.height / 2;

        // To make the list scrollable we need to calculate the starting index
        // The starting index is the index of the first element that we want to show in the list
        let starting_index = if self.selected_workspace >= (area.height as usize - 1) {
            self.selected_workspace
                .saturating_sub(area.height as usize - 1)
                + 1
        } else {
            self.selected_workspace
                .saturating_sub(area.height as usize - 1)
        };

        utils::border(&area, "WORKSPACER")?;
        let workspaces_to_render = self
            .workspaces
            .iter()
            .enumerate()
            .filter(|&(i, _)| i >= starting_index)
            .take(area.height as usize - 1);

        // Draw workspaces
        utils::reset_cursor_in_area(&area)?;
        for (index, workspace) in workspaces_to_render {
            utils::go_to_next_line_in_area(&area, 1)?;
            let mut styled_workspace = utils::build_line(
                &format!("{} <{}>", workspace.title, workspace.path),
                (area.width - 2) as usize,
            )
            .stylize();
            if index == self.selected_workspace {
                styled_workspace = styled_workspace.on(Color::White).with(Color::Black);
            }
            stdout().queue(PrintStyledContent(styled_workspace))?;
        }

        stdout().flush()?;

        Ok(())
    }

    pub fn handle_key_event(&mut self, code: KeyCode) {
        match code {
            KeyCode::Char('q') => self.quit = true,
            KeyCode::Enter => {
                let workspace = self.workspaces.get(self.selected_workspace);
                if let Err(error) = workspaces::exec_workspace(&self.config, workspace) {
                    eprintln!(
                        "An error occured while executing \"{}\" with workspace \"{}\"",
                        self.config.command,
                        workspace.unwrap_or(&Workspace::default())
                    );
                    eprintln!("{}", error);
                    std::process::exit(1);
                };
            }
            KeyCode::Up => {
                if self.workspaces.len() > 1 {
                    self.selected_workspace = if self.selected_workspace == 0 {
                        self.workspaces.len() - 1
                    } else {
                        self.selected_workspace - 1
                    };
                }
            }
            KeyCode::Down => {
                if self.workspaces.len() > 1 {
                    self.selected_workspace =
                        if self.selected_workspace == (self.workspaces.len() - 1) {
                            0
                        } else {
                            self.selected_workspace + 1
                        };
                }
            }
            _ => {}
        }
    }
}
