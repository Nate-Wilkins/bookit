extern crate assert_cmd;
extern crate predicates;
extern crate pretty_assertions;
extern crate tempfile;

use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::io::Read;
use std::{io::Write, process::Command};
use tempfile::tempdir;
use tempfile::NamedTempFile;

#[test]
fn test_command_completions_type_zsh() -> Result<(), Box<dyn std::error::Error>> {
    // Given the CLI.
    let mut cmd = Command::cargo_bin("bookit")?;

    // When the user generates completions for zsh.
    let result = cmd.arg("completions").arg("--type").arg("zsh").assert();

    result
        // Then no errors occurred.
        .success()
        .stderr(predicate::str::is_empty())
        // Then completions for zsh were outputted.
        .stdout(predicate::str::contains("#compdef bookit"));

    Ok(())
}

#[test]
fn test_command_config_create() -> Result<(), Box<dyn std::error::Error>> {
    // Given the CLI.
    let mut cmd = Command::cargo_bin("bookit")?;

    // And the user has a place to put the config.
    let config_path = tempdir().unwrap().path().join(".bookit");

    // When the user runs the command config create.
    let result = cmd.arg("config").arg("create").arg(&config_path).assert();

    result
        // Then no errors occurred.
        .success()
        .stderr(predicate::str::is_empty())
        // Then configuration file was created.
        .stdout(predicate::str::contains(format!(
            "Creating configuration at '{}'.
Created configuration.",
            &config_path.into_os_string().to_str().unwrap()
        )));

    Ok(())
}

#[test]
fn test_command_config_create_config_file_exists() -> Result<(), Box<dyn std::error::Error>> {
    // Given the CLI.
    let mut cmd = Command::cargo_bin("bookit")?;

    // And the user has an existing configuration file.
    let mut config_path = NamedTempFile::new()?;
    config_path
        .write_all(String::from("Invalid but don't overwrite.").as_bytes())
        .unwrap();

    // When the user runs the command config create.
    let result = cmd
        .arg("config")
        .arg("create")
        .arg(config_path.path().as_os_str().to_str().unwrap())
        .assert();

    result
        // Then a failure occurs with the correct errors.
        .failure()
        .stderr(predicate::str::contains(format!(
            "Configuration file already exists at '{}'.
",
            &config_path.path().as_os_str().to_str().unwrap()
        )))
        // Then there was no output.
        .stdout(predicate::str::is_empty());

    // Then the configuration file was *not* modified.
    let mut config_contents = String::new();
    config_path
        .reopen()
        .unwrap()
        .read_to_string(&mut config_contents)
        .unwrap();
    assert_eq!("Invalid but don't overwrite.", &config_contents);

    Ok(())
}

#[test]
fn test_command_view_provided_config_missing() -> Result<(), Box<dyn std::error::Error>> {
    // Given the CLI.
    let mut cmd = Command::cargo_bin("bookit")?;

    // And the configuration is missing.
    // When the user generates completions for zsh.
    let result = cmd.arg("--config").arg("./.bookit").arg("view").assert();

    result
        // Then a failure occurs with the correct errors.
        .failure()
        .stderr(predicate::str::contains("No config found at './.bookit'"))
        // Then there was no output.
        .stdout(predicate::str::is_empty());

    Ok(())
}

#[test]
fn test_command_view_bookmarks_empty() -> Result<(), Box<dyn std::error::Error>> {
    // Given the CLI.
    let mut cmd = Command::cargo_bin("bookit")?;

    // And there's a valid bookit configuration.
    // And the bookmarks are empty.
    let mut input_config_file = NamedTempFile::new()?;
    input_config_file.write_all(
        String::from(
            r#"---
bookmarks: {}"#,
        )
        .as_bytes(),
    )?;

    // When the user runs the command to view bookmarks.
    let result = cmd
        .arg("--config")
        .arg(input_config_file.path())
        .arg("view")
        .assert();

    result
        // Then no errors occurred.
        .success()
        .stderr(predicate::str::is_empty())
        // Then the correct output was printed.
        .stdout(predicate::str::contains(""));

    Ok(())
}

#[test]
fn test_command_view_bookmarks_one() -> Result<(), Box<dyn std::error::Error>> {
    // Given the CLI.
    let mut cmd = Command::cargo_bin("bookit")?;

    // And there's a valid bookit configuration.
    // And there is one bookmark.
    let mut input_config_file = NamedTempFile::new()?;
    input_config_file.write_all(
        String::from(
            r#"---
bookmarks:
  GitHub (bookit):
    url: "https://github.com/Nate-Wilkins/bookit"
    tags:
      - internet
      - browser
      - bookmarks"#,
        )
        .as_bytes(),
    )?;

    // When the user runs the command to view bookmarks.
    let result = cmd
        .arg("--config")
        .arg(input_config_file.path())
        .arg("view")
        .assert();

    result
        // Then no errors occurred.
        .success()
        .stderr(predicate::str::is_empty())
        // Then the correct output was printed.
        .stdout(predicate::str::contains("GitHub (bookit)\tinternet,browser,bookmarks\thttps://github.com/Nate-Wilkins/bookit\t\0icon\x1fgithub.com"));

    Ok(())
}

#[test]
fn test_command_view_bookmarks_multiple() -> Result<(), Box<dyn std::error::Error>> {
    // Given the CLI.
    let mut cmd = Command::cargo_bin("bookit")?;

    // And there's a valid bookit configuration.
    // And there is one bookmark.
    let mut input_config_file = NamedTempFile::new()?;
    input_config_file.write_all(
        String::from(
            r#"---
bookmarks:
  GitHub (bookit):
    url: "https://github.com/Nate-Wilkins/bookit"
    tags:
      - internet
      - browser
      - bookmarks
  GitHub (mallardscript):
    url: "https://github.com/Nate-Wilkins/mallardscript"
    tags:
      - duckyscript
      - security
      - keyboard
      - automation"#,
        )
        .as_bytes(),
    )?;

    // When the user runs the command to view bookmarks.
    let result = cmd
        .arg("--config")
        .arg(input_config_file.path())
        .arg("view")
        .assert();

    result
        // Then no errors occurred.
        .success()
        .stderr(predicate::str::is_empty())
        // Then the correct output was printed.
        .stdout(predicate::str::contains("GitHub (bookit)\tinternet,browser,bookmarks\thttps://github.com/Nate-Wilkins/bookit\t\0icon\x1fgithub.com
GitHub (mallardscript)\tduckyscript,security,keyboard,automation\thttps://github.com/Nate-Wilkins/mallardscript\t\0icon\x1fgithub.com"));

    Ok(())
}

#[test]
fn test_command_view_bookmarks_multiple_exclude_icon() -> Result<(), Box<dyn std::error::Error>> {
    // Given the CLI.
    let mut cmd = Command::cargo_bin("bookit")?;

    // And there's a valid bookit configuration.
    // And there is one bookmark.
    let mut input_config_file = NamedTempFile::new()?;
    input_config_file.write_all(
        String::from(
            r#"---
bookmarks:
  GitHub (bookit):
    url: "https://github.com/Nate-Wilkins/bookit"
    tags:
      - internet
      - browser
      - bookmarks
  GitHub (mallardscript):
    url: "https://github.com/Nate-Wilkins/mallardscript"
    tags:
      - duckyscript
      - security
      - keyboard
      - automation"#,
        )
        .as_bytes(),
    )?;

    // When the user runs the command to view bookmarks.
    let result = cmd
        .arg("--config")
        .arg(input_config_file.path())
        .arg("view")
        .arg("--exclude-icon")
        .assert();

    result
        // Then no errors occurred.
        .success()
        .stderr(predicate::str::is_empty())
        // Then the correct output was printed.
        .stdout(predicate::str::contains("GitHub (bookit)\tinternet,browser,bookmarks\thttps://github.com/Nate-Wilkins/bookit
GitHub (mallardscript)\tduckyscript,security,keyboard,automation\thttps://github.com/Nate-Wilkins/mallardscript"));

    Ok(())
}

#[test]
fn test_command_add_bookmark() -> Result<(), Box<dyn std::error::Error>> {
    // Given the CLI.
    let mut cmd = Command::cargo_bin("bookit")?;

    // And there's a valid bookit configuration.
    // And there is one bookmark.
    let mut input_config_file = NamedTempFile::new()?;
    input_config_file.write_all(
        String::from(
            r#"---
bookmarks:
  GitHub (bookit):
    url: "https://github.com/Nate-Wilkins/bookit"
    tags:
      - internet
      - browser
      - bookmarks"#,
        )
        .as_bytes(),
    )?;

    // When the user runs the command to add a bookmark.
    let result = cmd
        .arg("--config")
        .arg(input_config_file.path())
        .arg("add")
        .arg("--name")
        .arg("GitHub (mallardscript)")
        .arg("--url")
        .arg("https://github.com/Nate-Wilkins/mallardscript")
        .arg("--tags")
        .arg("duckyscript,security,keyboard,automation")
        .assert();

    result
        // Then no errors occurred.
        .success()
        .stderr(predicate::str::is_empty())
        // Then the correct output was printed.
        .stdout(predicate::str::contains(
            "Added bookmark 'GitHub (mallardscript)\thttps://github.com/Nate-Wilkins/mallardscript",
        ));

    Ok(())
}

#[test]
fn test_command_add_bookmark_exists() -> Result<(), Box<dyn std::error::Error>> {
    // Given the CLI.
    let mut cmd = Command::cargo_bin("bookit")?;

    // And there's a valid bookit configuration.
    // And there is one bookmark.
    let mut input_config_file = NamedTempFile::new()?;
    input_config_file.write_all(
        String::from(
            r#"---
bookmarks:
  GitHub (bookit):
    url: "https://github.com/Nate-Wilkins/bookit"
    tags:
      - internet
      - browser
      - bookmarks"#,
        )
        .as_bytes(),
    )?;

    // When the user runs the command to add a bookmark that already exists.
    let result = cmd
        .arg("--config")
        .arg(input_config_file.path())
        .arg("add")
        .arg("--name")
        .arg("GitHub (bookit)")
        .arg("--url")
        .arg("https://github.com/Nate-Wilkins/bookit")
        .arg("--tags")
        .arg("internet,browser,bookmarks")
        .assert();

    result
        // Then an error occurred.
        .failure()
        .stderr(predicate::str::contains(
            "Bookmark already exists with name 'GitHub (bookit)'. Use '--force' to override.",
        ))
        // Then no output was printed.
        .stdout(predicate::str::is_empty());

    Ok(())
}

#[test]
fn test_command_add_bookmark_exists_force() -> Result<(), Box<dyn std::error::Error>> {
    // Given the CLI.
    let mut cmd = Command::cargo_bin("bookit")?;

    // And there's a valid bookit configuration.
    // And there is one existing bookmark.
    let mut input_config_file = NamedTempFile::new()?;
    input_config_file.write_all(
        String::from(
            r#"---
bookmarks:
  GitHub (bookit):
    url: "https://github.com/Nate-Wilkins/bookit"
    tags:
      - internet
      - browser
      - bookmarks"#,
        )
        .as_bytes(),
    )?;

    // When the user runs the command to add a bookmark that already exists.
    let result = cmd
        .arg("--config")
        .arg(input_config_file.path())
        .arg("add")
        .arg("--name")
        .arg("GitHub (bookit)")
        .arg("--url")
        .arg("https://github.com/Nate-Wilkins/renamed-bookit")
        .arg("--tags")
        .arg("internet,browser,bookmarks")
        .arg("--force")
        .assert();

    result
        // Then no errors occurred.
        .success()
        .stderr(predicate::str::is_empty())
        // Then the correct output was printed.
        .stdout(predicate::str::contains(
            "Added bookmark 'GitHub (bookit)\thttps://github.com/Nate-Wilkins/renamed-bookit'.",
        ));

    Ok(())
}

#[test]
fn test_command_edit_bookmark_missing() -> Result<(), Box<dyn std::error::Error>> {
    // Given the CLI.
    let mut cmd = Command::cargo_bin("bookit")?;

    // And there's a valid bookit configuration.
    // And there are no bookmarks.
    let mut input_config_file = NamedTempFile::new()?;
    input_config_file.write_all(
        String::from(
            r#"---
bookmarks: {}"#,
        )
        .as_bytes(),
    )?;

    // When the user runs the command to edit a bookmark that doesn't exist.
    let result = cmd
        .arg("--config")
        .arg(input_config_file.path())
        .arg("edit")
        .arg("--name")
        .arg("GitHub (bookit)")
        .assert();

    result
        // Then an error occurred.
        .failure()
        .stderr(predicate::str::contains(
            "Bookmark 'GitHub (bookit)' not found.",
        ))
        // Then no output was printed.
        .stdout(predicate::str::is_empty());

    Ok(())
}

#[test]
fn test_command_edit_bookmark() -> Result<(), Box<dyn std::error::Error>> {
    // Given the CLI.
    let mut cmd = Command::cargo_bin("bookit")?;

    // And there's a valid `$BOOKIT_EDIT_COMMAND` environment variable.
    // NOTE: In this case we're just going to mock print the bookmark details.
    std::env::set_var(
        "BOOKIT_EDIT_COMMAND",
        "printf \"EDITOR \\\"$BOOKIT_CONFIG_PATH\\\" \\\"+/$BOOKIT_BOOKMARK_NAME\\\"\\n\"",
    );

    // And there's a valid bookit configuration.
    // And there is one bookmark.
    let mut input_config_file = NamedTempFile::new()?;
    input_config_file.write_all(
        String::from(
            r#"---
bookmarks:
  GitHub (bookit/issues):
    url: "https://github.com/Nate-Wilkins/bookit/issues"
    tags:
      - internet
      - browser
      - bookmarks
      - issues"#,
        )
        .as_bytes(),
    )?;

    // When the user runs the command to edit a bookmark.
    let result = cmd
        .arg("--config")
        .arg(input_config_file.path())
        .arg("edit")
        .arg("--name")
        .arg("GitHub (bookit/issues)")
        .assert();

    result
        // Then no errors occurred.
        .success()
        .stderr(predicate::str::is_empty())
        // Then the correct output was printed.
        .stdout(predicate::str::contains(format!(
            "EDITOR \"{}\" \"+/{}\"
Edited bookmark '{}'.",
            input_config_file.path().as_os_str().to_str().unwrap(),
            "GitHub (bookit/issues)",
            "GitHub (bookit/issues)"
        )));

    Ok(())
}

#[test]
fn test_command_delete_bookmark() -> Result<(), Box<dyn std::error::Error>> {
    // Given the CLI.
    let mut cmd = Command::cargo_bin("bookit")?;

    // And there's a valid bookit configuration.
    // And there is one bookmark.
    let mut input_config_file = NamedTempFile::new()?;
    input_config_file.write_all(
        String::from(
            r#"---
bookmarks:
  GitHub (bookit):
    url: "https://github.com/Nate-Wilkins/bookit"
    tags:
      - internet
      - browser
      - bookmarks"#,
        )
        .as_bytes(),
    )?;

    // When the user runs the command to delete a bookmark.
    let result = cmd
        .arg("--config")
        .arg(input_config_file.path())
        .arg("delete")
        .arg("--name")
        .arg("GitHub (bookit)")
        .assert();

    result
        // Then no errors occurred.
        .success()
        .stderr(predicate::str::is_empty())
        // Then the correct output was printed.
        .stdout(predicate::str::contains(
            "Deleted bookmark 'GitHub (bookit)'.",
        ));

    // Then the configuration file was modified correctly.
    let mut config_contents = String::new();
    input_config_file
        .reopen()
        .unwrap()
        .read_to_string(&mut config_contents)
        .unwrap();
    assert_eq!(
        "---
bookmarks: {}
",
        &config_contents
    );

    Ok(())
}

#[test]
fn test_command_delete_bookmark_missing() -> Result<(), Box<dyn std::error::Error>> {
    // Given the CLI.
    let mut cmd = Command::cargo_bin("bookit")?;

    // And there's a valid bookit configuration.
    // And there is one bookmark.
    let mut input_config_file = NamedTempFile::new()?;
    input_config_file.write_all(
        String::from(
            r#"---
bookmarks:
  GitHub (bookit):
    url: "https://github.com/Nate-Wilkins/bookit"
    tags:
      - internet
      - browser
      - bookmarks"#,
        )
        .as_bytes(),
    )?;

    // When the user runs the command to delete a bookmark that doesn't exist.
    let result = cmd
        .arg("--config")
        .arg(input_config_file.path())
        .arg("delete")
        .arg("--name")
        .arg("GitHub (mallardscript)")
        .assert();

    result
        // Then an error occurred.
        .failure()
        .stderr(predicate::str::contains(
            "Bookmark doesn't exist with name 'GitHub (mallardscript)'.",
        ))
        // Then no output was printed.
        .stdout(predicate::str::is_empty());

    // Then the configuration file was *not* modified.
    let mut config_contents = String::new();
    input_config_file
        .reopen()
        .unwrap()
        .read_to_string(&mut config_contents)
        .unwrap();
    assert_eq!(
        r#"---
bookmarks:
  GitHub (bookit):
    url: "https://github.com/Nate-Wilkins/bookit"
    tags:
      - internet
      - browser
      - bookmarks"#,
        &config_contents
    );

    Ok(())
}
