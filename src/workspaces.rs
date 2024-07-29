use std::{
    fs::OpenOptions,
    io::{BufRead, BufReader},
    path::Path,
};

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
