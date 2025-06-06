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
                if let Some((key, value)) = line.split_once('=') {
                    match key.trim() {
                        "command" => config.command = value.trim().to_string(),
                        "workspaces" => config.workspaces = value.trim().to_string(),
                        _ => {}
                    }
                }
            }
        };

        config
    }
}
