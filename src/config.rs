use std::{
    fs::{self, OpenOptions},
    io::{BufRead, BufReader, Write},
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

        if !config_directory.exists() || !config_file_path.exists() {
            let _ = fs::create_dir(&config_directory);
            if config_directory.exists() {
                if let Ok(mut config_file) = OpenOptions::new()
                    .create(true)
                    .truncate(true)
                    .write(true)
                    .open(&config_file_path)
                {
                    if config_file
                        .write_all(
                            format!(
                                "command={}\nworkspaces={}\n",
                                config.command, config.workspaces
                            )
                            .as_bytes(),
                        )
                        .is_err()
                    {
                        // TODO: Create an actual log line to print this into
                        // eprintln!("Could not create default configuration");
                    }
                };
            }
        }

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
