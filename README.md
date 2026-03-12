# fix-steam-blank-icon-rs

A Rust rewrite of [fix-steam-blank-icon](https://github.com/mjawood/fix-steam-blank-icon). Fixes blank icons for Steam game shortcuts.

Ref: https://github.com/mrsimb/steam_blank_icon_fix

## Usage

```cmd
fix-steam-blank-icon-rs game1.url game2.url ...
```

Glob patterns are supported:

```cmd
fix-steam-blank-icon-rs *.url
```

Use `--dry-run` to preview what would be downloaded without actually downloading:

```cmd
fix-steam-blank-icon-rs --dry-run *.url
```

## What it does

- One reason the game shortcut icon turns into a blank sheet is because the `.ico` file in the Steam installation directory is missing.
  * This can happen if Steam is uninstalled while keeping the game library on another drive.
  * To see exactly which `.ico` file is being referenced, you can open the shortcut with a text editor like notepad.exe.
- From the end of the shortcut URL, you can determine the `gameid`.
- The desired icon can actually be obtained by accessing `https://steamdb.info/app/{gameid}/info/`. Download it and place it where it should be.
  * This script directly refers to the icon URL at `https://cdn.cloudflare.steamstatic.com/steamcommunity/public/images/apps/{gameid}/{icon_name}`, but it may change in the future.

## Building

```cmd
cargo build --release
```

The binary will be at `target/release/fix-steam-blank-icon-rs.exe`.
