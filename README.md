Workspacer
==========

Workspace manager for the terminal

Build
-----

This application depends on the following libraries:

* [ftxui](https://github.com/ArthurSonzogni/FTXUI)
* [neovim](https://github.com/neovim/neovim)
* [vim](https://github.com/vim/vim) (optional)
* [helix](https://github.com/helix-editor/helix/) (optional)

You can build this application with:

```bash
git clone git@github.com:blinxen/workspacer.git
cd workspacer/
mkdir build
cmake -S . -B build/
cmake --build build/
```

After building, you should be able to find the binary called `workspacer` in
the `build` directory.
