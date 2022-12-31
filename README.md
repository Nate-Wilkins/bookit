# Bookit

> Fast and simple bookmark manager for your operating system.

## TODO:

- Badges
- Tests
- crates.io -> bookit

## Install

```
cargo install bookit
```

## Configuration

### Shell Completions

Bookit supports shell completions. To take advantage of them you can generate yours with:

```
bookit completions --type $SHELL               # Where $SHELL is zsh,bash,fish,elvish,powershell
```

### Environment Variables

To configure bookit you can update the following variables.

- `$BOOKIT_LOG_LEVEL` (unset): Sets the log level for the program.

- `$BOOKIT_CONFIG_PATH` (`~/.bookit`):
   Configuration file path where bookit stores bookmarks.

- `$BOOKIT_EDIT_COMMAND` (`$EDITOR +/$BOOKIT_BOOKMARK_NAME '$BOOKIT_CONFIG_PATH'`):
  Process command to run to edit a bookmark.

## Development

Written in rust. You can build and install with the following:

```
cargo build
cargo install --path .
```

