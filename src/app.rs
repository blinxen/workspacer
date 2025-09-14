use crate::buffer::Buffer;
use crate::config::Config;
use crate::utils::Rect;
use crate::workspaces::{self, Workspace};
use crate::{terminal, utils};
use crossterm::event::{KeyCode, KeyModifiers};
use crossterm::style::{ContentStyle, StyledContent, Stylize};
use crossterm::{ExecutableCommand, QueueableCommand, cursor, execute};
use nucleo::Nucleo;
use nucleo::pattern::{CaseMatching, Normalization};
use std::io::Error as IOError;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

fn init_matcher(config: &Config) -> Nucleo<Workspace> {
    let workspaces = workspaces::read_workspaces(&config.workspaces);
    let matcher = Nucleo::new(
        nucleo::Config::DEFAULT,
        Arc::new(|| {}),
        None,
        // Name + path
        2,
    );
    let injector = matcher.injector();
    // TODO: Remove clones
    for workspace in workspaces.clone() {
        injector.push(workspace, |workspace, cols| {
            cols[0] = workspace.title.clone().into();
            cols[1] = workspace.path.clone().into();
        });
    }

    matcher
}

pub struct App {
    config: Config,
    selected_workspace: usize,
    error_line: String,
    error_line_reset_time: Duration,
    buffer: Buffer,
    search_query: String,
    matcher: Nucleo<Workspace>,
    pub quit: bool,
}

impl App {
    pub fn new() -> Self {
        let config = Config::read();
        App {
            matcher: init_matcher(&config),
            config,
            selected_workspace: 0,
            error_line: String::new(),
            error_line_reset_time: Duration::new(0, 0),
            buffer: Buffer::new(terminal::size()),
            quit: false,
            search_query: String::new(),
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
        let mut top_line_offset = 0;
        let area = terminal::size();
        self.matcher.tick(10);
        self.buffer.resize(&area);
        self.buffer.stdout.queue(cursor::Hide)?;

        // Draw border
        utils::border(
            &mut self.buffer,
            &area,
            true,
            String::from("WORKSPACER"),
            None,
            Some(self.error_line.clone().red()),
        );
        top_line_offset += 1;

        // Draw search line
        let search_line_y = area.y + top_line_offset;
        self.buffer.write_string(
            area.x + 1,
            search_line_y,
            StyledContent::new(ContentStyle::default(), self.search_query.clone()),
        );
        top_line_offset += 1;
        self.buffer.write_string(
            area.x + 1,
            area.y + top_line_offset,
            StyledContent::new(
                ContentStyle::default().dark_grey(),
                utils::build_line("─", area.width as usize, "─"),
            ),
        );

        // Draw workspaces
        for (index, workspace) in self.workspaces_to_render(&area) {
            top_line_offset += 1;
            let style = if index == self.selected_workspace {
                ContentStyle::default().black().on_white()
            } else {
                ContentStyle::default().reset()
            };
            self.buffer.write_string(
                area.x + 1,
                area.y + top_line_offset,
                StyledContent::new(
                    style,
                    utils::build_line(
                        &format!("{} <{}>", workspace.title, workspace.path),
                        area.width as usize - 2,
                        " ",
                    ),
                ),
            );
        }

        let key_bindings = [
            "[CTRL + q] Quit",
            "[<ENTER>] Enter workspace",
            "[CTRL + e] Edit workspaces",
            "[CTRL + w] Clear search",
        ];
        let mut start = area.x + 1;
        for (index, key) in key_bindings.iter().enumerate() {
            self.buffer.write_string(
                start,
                area.y + area.height - 1,
                StyledContent::new(ContentStyle::default().white(), String::from(*key)),
            );
            start += key.len() as u16 + index as u16 + 2;
        }

        self.buffer.flush()?;
        self.buffer.stdout.execute(cursor::MoveTo(
            area.x + 1 + self.search_query.len() as u16,
            search_line_y,
        ))?;
        self.buffer.stdout.execute(cursor::Show)?;

        Ok(())
    }

    pub fn handle_key_event(
        &mut self,
        code: KeyCode,
        modifiers: KeyModifiers,
    ) -> Result<(), IOError> {
        if modifiers == KeyModifiers::CONTROL {
            match code {
                KeyCode::Char('q') => self.quit = true,
                KeyCode::Char('w') => {
                    self.search_query.clear();
                    self.parse_search_query(false);
                }
                KeyCode::Char('e') => {
                    self.buffer.reset();
                    terminal::restore_terminal()?;
                    if self.config.edit_workspaces() {
                        self.matcher = init_matcher(&self.config);
                    } else {
                        self.log_error("Could not open / save workspaces file");
                    }
                    self.selected_workspace = 0;
                    terminal::prepare_terminal()?;
                }
                _ => {}
            }
        } else {
            match code {
                KeyCode::Enter => {
                    let workspace = self
                        .matcher
                        .snapshot()
                        .get_matched_item(self.selected_workspace as u32);
                    if let Some(workspace) = workspace {
                        self.buffer.reset();
                        terminal::restore_terminal()?;
                        if let Err(error) = workspaces::exec_workspace(&self.config, workspace.data)
                        {
                            self.log_error(&error.to_string());
                        };
                        terminal::prepare_terminal()?;
                    }
                }
                KeyCode::Up => {
                    if self.matcher.snapshot().matched_item_count() > 1 {
                        self.selected_workspace = if self.selected_workspace == 0 {
                            self.matcher.snapshot().matched_item_count() as usize - 1
                        } else {
                            self.selected_workspace - 1
                        };
                    }
                }
                KeyCode::Down => {
                    if self.matcher.snapshot().matched_item_count() > 1 {
                        self.selected_workspace = if self.selected_workspace
                            == (self.matcher.snapshot().matched_item_count() as usize - 1)
                        {
                            0
                        } else {
                            self.selected_workspace + 1
                        };
                    }
                }
                KeyCode::Backspace => {
                    self.search_query.pop();
                    self.parse_search_query(false);
                }
                KeyCode::Char(c) => {
                    self.search_query.push(c);
                    self.parse_search_query(true);
                }
                _ => {}
            }
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

    fn workspaces_to_render(&self, area: &Rect) -> Vec<(usize, Workspace)> {
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

        self.matcher
            .snapshot()
            .matched_items(..)
            // TODO: I don't like the clone here
            .map(|w| w.data.clone())
            .enumerate()
            .filter(|&(i, _)| i >= starting_index)
            .take(area.height as usize - 1)
            .collect()
    }

    fn parse_search_query(&mut self, append: bool) {
        self.selected_workspace = 0;
        self.matcher.pattern.reparse(
            0,
            &self.search_query,
            CaseMatching::Ignore,
            Normalization::Smart,
            append,
        );
        self.matcher.pattern.reparse(
            1,
            &self.search_query,
            CaseMatching::Ignore,
            Normalization::Smart,
            append,
        );
    }
}
