# Docket

> Fast and simple bookmark manager for your operating system.

## Configuration

TO configure docket you can update the following variables.

- `$DOCKET_LOG_LEVEL` (unset): Sets the log level for the program.

- `$DOCKET_CONFIG_PATH` (`~/.docket`):
   Configuration file path where docket stores bookmarks.

- `$DOCKET_EDIT_COMMAND` (`$EDITOR +/$DOCKET_BOOKMARK_NAME '$DOCKET_CONFIG_PATH'`):
  Process command to run to edit a bookmark.

## Development

Written in rust. You can build and install with the following:

```
cargo build
cargo install --path .
```

