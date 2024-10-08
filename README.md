Workspacer
==========

Workspacer is a workspace manager for the terminal.
It was inspired by [telescope-project.nvim](https://github.com/nvim-telescope/telescope-project.nvim)
but without the requirement on a specific terminal editor.

Build
-----

```bash
cargo build --release
```

Configuration
-------------

The configuration file is held very minimal and looks like this:

```
command=/usr/bin/vim
workspaces=/home/foo/.config/workspacer/workspaces
```

Each configuration option is written on a single line.
The equals sign (`=`) is used to separate the configuration from its value.

By default, `vim` is called once a project is selected.

### Workspaces

The workspaces file is composed of absolute paths to the respective workspace.
A workspace, for now, is considered a single directory.

Example workspaces file:

```
/home/foo/workspace1
/home/foo/workspace2
/home/foo/workspace3
```

Invalid file paths are ignored.
