# Bookit

![Version](https://img.shields.io/crates/v/bookit?style=flat-square)
![Build](https://img.shields.io/travis/Nate-Wilkins/bookit/main?style=flat-square)
![Downloads](https://img.shields.io/crates/d/bookit?color=%230E0&style=flat-square)
![Open Issues](https://img.shields.io/github/issues-raw/Nate-Wilkins/bookit?style=flat-square)
![License](https://img.shields.io/github/license/Nate-Wilkins/bookit?color=%2308F&style=flat-square)

> Fast and simple bookmark manager for your operating system.

## Install

```
cargo install bookit
```

## Configuration

### Bookmarks

To setup `bookit` you need to run:

```
bookit config create
```

This will create a configuration file for where your bookmarks will be stored.

### Shell Completions

You can put this in your `.zshrc` file (just make sure `$HOME/.zsh_functions/` is in your
`fpath`):

```
if [[ ! -f "$HOME/.zsh_functions/_bookit" ]]; then
  bookit completions --type zsh > "$HOME/.zsh_functions/_bookit"
fi
```

Or you can generate yours with:

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

Written in rust. Workflows are defined in `.envrc.sh`.
