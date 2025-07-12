# balatro-tui

A balatro mod manager and helper for the terminal.

Written in Rust with the Ratatui library.

Currently only supports Linux, but support for Windows (+macos, maybe) is planned.

Uses the [Balatro Mod Index](https://github.com/skyline69/balatro-mod-index) repository for downloading mods.

### Currently implemented
- Viewing mod list
- Launching the game
- Installing mods from the Balatro Mod Index

### Planned (in order of when they will probably be added)
- Updating mods and Steamodded (+ lovely?)
- Helper scripts for making mods
- Windows support (already implemented but untested and uncompiled)

### Why?

The traditional Balatro Mod Manager does not have good Linux support. This vexes me as a Linux user.
So I decided to make my own mod manager from scratch rather than patching BMM to work
(even though that's also something I did, although they haven't accepted my pull request yet).
Anyway, TL;DR, much better linux support.

As for why it's a terminal application, I spend a lot of time doing `./run_balatro.sh` in the terminal when I am
developing mods and having a launcher for that sounded pretty nice to me, plus I wanted to add some extra scripts
for helping with developing mods (ie, automatically making 2x versions of all textures), which would be much easier
to use in a terminal rather than as a standalone app. Plus it's *fast* this way, compared to using Tauri.