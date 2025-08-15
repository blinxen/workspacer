Workspacer
==========

Workspacer is a workspace manager for the terminal.
It was inspired by [telescope-project.nvim](https://github.com/nvim-telescope/telescope-project.nvim)
but without the requirement on a specific terminal editor.

Installation
------------

### Release tarball

```bash
wget https://github.com/blinxen/workspacer/releases/download/$VERSION/workspacer.tar.gz
```

Configuration
-------------

The configuration file is held very minimal and looks like this:

```
command=/usr/bin/vim
edit_command=/usr/bin/vim
workspaces=/home/foo/.config/workspacer/workspaces
```

Each configuration option is written on a single line.
The equals sign (`=`) is used to separate the configuration from its value.

By default, `vim` is called once a project is selected.

*NOTE*: Command paths don't have to be absolute or relative paths.
They can be anything that is found under `PATH` too.

### Configuration options

* `command`: Command that should be executed on a workspace when `<ENTER>` key was pressed
* `edit_command`: Command that should be used to open and edit the workspaces file
* `workspaces`: Path to the workspaces file

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

License
-------

The source code is primarily distributed under the terms of the MIT License.
See LICENSE for details.
