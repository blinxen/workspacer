use std::{
    fs::OpenOptions,
    io::{BufRead, BufReader},
    path::PathBuf,
};

pub struct Config {
    // Command to call after a project was selected
    pub command: String,
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
            workspaces: config_directory
                .join("workspaces")
                .to_string_lossy()
                .to_string(),
        };

        if let Ok(config_file) = OpenOptions::new().read(true).open(config_file_path) {
            let file = BufReader::new(config_file);
            for line in file.lines().map_while(Result::ok) {
                let mut split = line.split('=');
                if let Some("command") = split.next() {
                    if let Some(command) = split.next() {
                        config.command = command.to_string();
                    }
                } else if let Some("workspaces") = split.next() {
                    if let Some(workspaces_path) = split.next() {
                        config.workspaces = workspaces_path.to_string();
                    }
                }
            }
        };

        config
    }
}
