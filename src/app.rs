use crate::buffer::Buffer;
use crate::config::Config;
use crate::workspaces::{self, Workspace};
use crate::{terminal, utils};
use crossterm::event::KeyCode;
use crossterm::style::{ContentStyle, StyledContent, Stylize};
use std::io::Error as IOError;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub struct App {
    workspaces: Vec<Workspace>,
    config: Config,
    selected_workspace: usize,
    error_line: String,
    error_line_reset_time: Duration,
    buffer: Buffer,
    pub quit: bool,
}

impl App {
    pub fn new() -> Self {
        let config = Config::read();
        App {
            workspaces: workspaces::read_workspaces(&config.workspaces),
            config,
            selected_workspace: 0,
            error_line: String::new(),
            error_line_reset_time: Duration::new(0, 0),
            buffer: Buffer::new(terminal::size()),
            quit: false,
        }
    }

    fn log_error(&mut self, error: &str) {
        self.error_line_reset_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::new(0, 0))
            + Duration::from_secs(5);
        self.error_line.push_str(error);
    }

    pub fn render(&mut self) -> Result<(), IOError> {
        self.buffer.resize(terminal::size());

        // Draw border
        utils::border(
            &mut self.buffer,
            String::from("WORKSPACER"),
            Some(
                String::from("q: Quit | <enter>: Enter workspace | e: Edit workspaces")
                    .black()
                    .on_white(),
            ),
            Some(self.error_line.clone().red()),
        );

        // Draw workspaces
        for (index, workspace) in self.workspaces_to_render() {
            let style = if index == self.selected_workspace {
                ContentStyle::default().black().on_white()
            } else {
                ContentStyle::default().reset()
            };
            self.buffer.write_string(
                self.buffer.area.x + 1,
                self.buffer.area.y + index as u16 + 2,
                StyledContent::new(
                    style,
                    utils::build_line(
                        format!("{} <{}>", workspace.title, workspace.path),
                        self.buffer.area.width as usize - 2,
                    ),
                ),
            );
        }

        self.buffer.flush()
    }

    pub fn handle_key_event(&mut self, code: KeyCode) -> Result<(), IOError> {
        match code {
            KeyCode::Char('q') => self.quit = true,
            KeyCode::Enter => {
                let workspace = self.workspaces.get(self.selected_workspace);
                terminal::restore_terminal()?;
                if let Err(error) = workspaces::exec_workspace(&self.config, workspace) {
                    self.log_error(&error.to_string());
                };
                terminal::prepare_terminal()?;
            }
            KeyCode::Char('e') => {
                terminal::restore_terminal()?;
                if self.config.edit_workspaces() {
                    self.workspaces = workspaces::read_workspaces(&self.config.workspaces);
                } else {
                    self.log_error("Could not open / save workspaces file");
                }
                terminal::prepare_terminal()?;
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

        if self.error_line_reset_time
            < SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("ERROR REAEDING SYSTEM TIME")
        {
            self.error_line.clear();
        }

        Ok(())
    }

    fn workspaces_to_render(&self) -> Vec<(usize, Workspace)> {
        // To make the list scrollable we need to calculate the starting index
        // The starting index is the index of the first element that we want to show in the list
        let starting_index = if self.selected_workspace >= (self.buffer.area.height as usize - 1) {
            self.selected_workspace
                .saturating_sub(self.buffer.area.height as usize - 1)
                + 1
        } else {
            self.selected_workspace
                .saturating_sub(self.buffer.area.height as usize - 1)
        };

        self.workspaces
            // TODO: I don't like the clone here
            .clone()
            .into_iter()
            .enumerate()
            .filter(|&(i, _)| i >= starting_index)
            .take(self.buffer.area.height as usize - 1)
            .collect()
    }
}
