use std::fs::OpenOptions;
use std::io::BufRead;
use std::io::BufReader;
use std::path::PathBuf;
use std::process::Command;

pub struct Config {
    // Command to execute after a project was selected
    pub command: String,
    // Edit command that will be executed to edit the workspaces file
    edit_command: String,
    // Path to the workspaces file that contains all workspaces
    pub workspaces: String,
}

impl Config {
    pub fn read() -> Config {
        let home = std::env::var_os("HOME")
            .map(PathBuf::from)
            .expect("Environment variable HOME is not defined");
        let config_directory = home.join(".config/workspacer");
        let config_file_path = home.join(".config/workspacer/config");
        let mut config = Config {
            command: String::from("/usr/bin/vim"),
            edit_command: String::from("/usr/bin/vim"),
            workspaces: config_directory
                .join("workspaces")
                .to_string_lossy()
                .to_string(),
        };

        if let Ok(config_file) = OpenOptions::new().read(true).open(config_file_path) {
            let file = BufReader::new(config_file);
            for line in file.lines().map_while(Result::ok) {
                if let Some((key, value)) = line.split_once('=') {
                    match key.trim() {
                        "command" => config.command = value.trim().to_string(),
                        "workspaces" => config.workspaces = value.trim().to_string(),
                        "edit_command" => config.edit_command = value.trim().to_string(),
                        _ => {}
                    }
                }
            }
        };

        config
    }

    pub fn edit_workspaces(&self) -> bool {
        Command::new(&self.edit_command)
            .arg(&self.workspaces)
            .status()
            .is_ok()
    }
}
