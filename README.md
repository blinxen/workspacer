Workspacer
==========

Workspace manager for the terminal

Build
-----

```bash
cargo build --release
```

Configuration
-------------

The configuration file is held very minimal and looks like this:

```
command=/usr/bin/hx
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
