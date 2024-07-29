use std::{
    fs::OpenOptions,
    io::{BufRead, BufReader},
    path::Path,
    process::Command,
};

use crate::{config::Config, terminal};

#[derive(Debug)]
pub struct Workspace {
    pub title: String,
    pub path: String,
}

pub fn read_workspaces(path: &str) -> Vec<Workspace> {
    let mut workspaces = Vec::new();

    if let Ok(config_file) = OpenOptions::new().read(true).open(path) {
        let file = BufReader::new(config_file);
        for line in file.lines().map_while(Result::ok) {
            let path = Path::new(&line);
            if path.exists() && path.file_name().is_some() {
                let file_path = path.file_name().unwrap().to_string_lossy().to_string();
                workspaces.push(Workspace {
                    title: file_path,
                    path: line,
                });
            }
        }
    }

    workspaces
}

pub fn exec_workspace(
    config: &Config,
    workspace: Option<&Workspace>,
) -> Result<(), std::io::Error> {
    if let Some(workspace) = workspace {
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
    }

    Ok(())
}
