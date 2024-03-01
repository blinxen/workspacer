Workspacer
==========

Workspace manager for the terminal

Build
-----

This application depends on the following libraries:

* [ftxui](https://github.com/ArthurSonzogni/FTXUI)

You can build this application with:

```bash
git clone git@github.com:blinxen/workspacer.git
cd workspacer/
mkdir build
cmake -S . -B build/
make -C build/
```

After building, you should be able to find the binary called `workspacer` in
the `build` directory.

Configuration
-------------

The configuration file is held very minimal and looks like this:

```
editor=/usr/bin/hx
workspaces=$HOME/.config/workspacer/workspaces
```

Each configuration option is written on a single line.
The equals sign (`=`) is used to separate the configuration from its value.

The workspaces file is composed of absolute paths to the respective workspace.
A workspace, for now, is considered a single directory.

Example workspaces file:

```
/home/foo/workspace1
/home/foo/workspace2
/home/foo/workspace3
```
