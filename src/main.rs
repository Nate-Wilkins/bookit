extern crate anyhow;
extern crate fsutils;
extern crate regex;

use anyhow::{bail, Context, Result};
use regex::Regex;
use std::{fs::File, io::Write, path::PathBuf};

#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
struct Config {
    bookmarks: std::collections::BTreeMap<String, ConfigBookmark>,
}

#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
struct ConfigBookmark {
    url: String,
    tags: Vec<String>,
}

/// Bookmarks manager.
fn main() {
    // Command line interface.
    let args = create_application().get_matches();

    // Initialize logger.
    initialize_logger();

    // Run the application.
    if let Err(e) = run(args) {
        eprintln!("{}", e);
        std::process::exit(2);
    }
    std::process::exit(0);
}

/// Create the application command line interface.
fn create_application() -> clap::App<'static, 'static> {
    return clap::App::new(clap::crate_name!())
        .bin_name(clap::crate_name!())
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about(clap::crate_description!())
        .setting(clap::AppSettings::ArgRequiredElseHelp)
        .subcommand(
            clap::SubCommand::with_name("config")
                .about("configuration")
                .subcommand(
                    clap::SubCommand::with_name("create").arg(
                        clap::Arg::with_name("create_config")
                            .takes_value(true)
                            .required(true)
                            .env("BOOKIT_CONFIG_PATH")
                            .default_value("~/.bookit")
                            .help("creates a configuration file"),
                    ),
                ),
        )
        .arg(
            clap::Arg::with_name("config")
                .global(true)
                .long("config")
                .required(false)
                .env("BOOKIT_CONFIG_PATH")
                .default_value("~/.bookit")
                .help("configuration file to use"),
        )
        .subcommand(
            clap::SubCommand::with_name("completions")
                .about("completions")
                .arg(
                    clap::Arg::with_name("type")
                        .short("t")
                        .long("type")
                        .required(true)
                        .takes_value(true)
                        .possible_values(&["Bash", "Elvish", "Fish", "PowerShell", "Zsh"])
                        .case_insensitive(true),
                ),
        )
        .subcommand(
            clap::SubCommand::with_name("view")
                .about("view bookmarks")
                .arg(
                    clap::Arg::with_name("exclude-icon")
                        .long("exclude-icon")
                        .required(false)
                        .takes_value(false)
                        .help("include icon for bookmarks"),
                ),
        )
        // TODO: Support for 'list-tags'.
        //.subcommand(clap::SubCommand::with_name("list-tags").about("lists all tags recorded"))
        .subcommand(
            clap::SubCommand::with_name("add")
                .about("add a new bookmark")
                .arg(
                    clap::Arg::with_name("name")
                        .short("n")
                        .long("name")
                        .required(true)
                        .takes_value(true)
                        .help("name of the bookmark"),
                )
                .arg(
                    clap::Arg::with_name("url")
                        .short("u")
                        .long("url")
                        .required(true)
                        .takes_value(true)
                        .help("url of the bookmark"),
                )
                .arg(
                    clap::Arg::with_name("tags")
                        .short("t")
                        .long("tags")
                        .required(true)
                        .multiple(true)
                        .takes_value(true)
                        .help("tags of the bookmark"),
                )
                .arg(
                    clap::Arg::with_name("force")
                        .long("force")
                        .required(false)
                        .takes_value(false)
                        .help("override a bookmark if one already exists"),
                ),
        )
        .subcommand(
            clap::SubCommand::with_name("edit")
                .about("edit a bookmark")
                .arg(
                    clap::Arg::with_name("name")
                        .short("n")
                        .long("name")
                        .required(true)
                        .takes_value(true)
                        .help("name of the bookmark"),
                ),
        )
        .subcommand(
            clap::SubCommand::with_name("delete")
                .about("delete a bookmark")
                .arg(
                    clap::Arg::with_name("name")
                        .short("n")
                        .long("name")
                        .required(true)
                        .takes_value(true)
                        .help("name of the bookmark"),
                ),
        );
}

/// Initializes the application logger.
fn initialize_logger() {
    // TODO: Support `--verbosity`.
    let env = env_logger::Env::default().filter("BOOKIT_LOG_LEVEL");

    return env_logger::Builder::from_env(env)
        .target(env_logger::Target::Stdout)
        .init();
}

/// Run application according to command line interface arguments.
fn run(args: clap::ArgMatches) -> Result<()> {
    if args.subcommand_matches("completions").is_some() {
        command_completions(args)?;
    } else if args.subcommand_matches("config").is_some() {
        command_config_create(args)?;
    } else if args.subcommand_matches("view").is_some() {
        command_view(args)?;
    } else if args.subcommand_matches("list-tags").is_some() {
        command_list_tags()?;
    } else if args.subcommand_matches("add").is_some() {
        command_add(args)?;
    } else if args.subcommand_matches("edit").is_some() {
        command_edit(args)?;
    } else if args.subcommand_matches("delete").is_some() {
        command_delete(args)?;
    }

    Ok(())
}

const REGEX_HOSTNAME: &str = r"^([^:]*://)([^/]*)/?.*?$";

/// Command to output completions of a specific type to STDOUT.
fn command_completions(args: clap::ArgMatches) -> Result<()> {
    // Parse arguments.
    let args_completions = args.subcommand_matches("completions").unwrap();
    let completion_type = args_completions.value_of("type").unwrap();

    // Generate completion.
    if completion_type == "bash" {
        create_application().gen_completions_to(
            create_application().get_bin_name().unwrap(),
            clap::Shell::Bash,
            &mut std::io::stdout(),
        );
    } else if completion_type == "elvish" {
        create_application().gen_completions_to(
            create_application().get_bin_name().unwrap(),
            clap::Shell::Elvish,
            &mut std::io::stdout(),
        );
    } else if completion_type == "fish" {
        create_application().gen_completions_to(
            create_application().get_bin_name().unwrap(),
            clap::Shell::Fish,
            &mut std::io::stdout(),
        );
    } else if completion_type == "powershell" {
        create_application().gen_completions_to(
            create_application().get_bin_name().unwrap(),
            clap::Shell::PowerShell,
            &mut std::io::stdout(),
        );
    } else if completion_type == "zsh" {
        create_application().gen_completions_to(
            create_application().get_bin_name().unwrap(),
            clap::Shell::Zsh,
            &mut std::io::stdout(),
        );
    } else {
        bail!("Completion type '{}' not supported.", completion_type);
    }

    Ok(())
}

// Command to create a configuration file.
fn command_config_create(args: clap::ArgMatches) -> Result<()> {
    // Parse arguments.
    let args_config = args.subcommand_matches("config").unwrap();
    let args_config_create = args_config.subcommand_matches("create").unwrap();
    let args_config_create_config = args_config_create.value_of("create_config").unwrap();

    // Get config path.
    let config_path =
        &std::path::PathBuf::from(shellexpand::tilde(args_config_create_config).into_owned());

    // Does the config already exist?
    if config_path.exists() {
        bail!(
            "Configuration file already exists at '{}'.",
            config_path.display()
        );
    }

    // Create the configuration file.
    println!("Creating configuration at '{}'.", &config_path.display());

    // Create path.
    let mut config_directory_path = PathBuf::from(config_path);
    config_directory_path.pop();
    fsutils::mkdir(config_directory_path.as_os_str().to_str().unwrap());

    // Create file.
    let mut config_file = File::options()
        .create_new(true)
        .read(true)
        .write(true)
        .open(config_path)?;
    config_file.write_all(
        String::from(
            r#"---
bookmarks: {}"#,
        )
        .as_bytes(),
    )?;

    println!("Created configuration.");

    Ok(())
}

/// Command to view bookmarks.
fn command_view(args: clap::ArgMatches) -> Result<()> {
    // Parse arguments.
    let args_config = args.value_of("config").unwrap();
    let args_view = args.subcommand_matches("view").unwrap();
    let args_view_exclude_icon = args_view.is_present("exclude-icon");

    // Load config.
    let config_path = &std::path::PathBuf::from(shellexpand::tilde(args_config).into_owned());
    let config = load_config(config_path)?;

    // Print out every bookmark with corresponding context.
    for bookmark in config.bookmarks.iter() {
        let re = Regex::new(REGEX_HOSTNAME).unwrap();
        if let Some(captures) = re.captures(&bookmark.1.url) {
            let hostname = captures.get(2).unwrap().as_str();

            println!(
                "{}\t{}\t{}{}",
                bookmark.0,
                bookmark.1.tags.join(","),
                bookmark.1.url,
                if !args_view_exclude_icon {
                    format!("\t\0icon\x1f{}", hostname)
                } else {
                    String::from("")
                },
            );
        } else {
            bail!(
                "Cannot parse bookmark entry '{}' not a valid entry.",
                &bookmark.1.url
            )
        }
    }

    Ok(())
}

/// Command to list out bookmark tags.
fn command_list_tags() -> Result<()> {
    // TODO@nw:
    Ok(())
}

/// Command to add a bookmark.
fn command_add(args: clap::ArgMatches) -> Result<()> {
    // Parse arguments.
    let args_add = args.subcommand_matches("add").unwrap();

    // Load config.
    let config_path = &std::path::PathBuf::from(
        shellexpand::tilde(args.value_of("config").unwrap()).into_owned(),
    );
    let mut config = load_config(config_path)?;

    // Parse bookmark details.
    let name = args_add.value_of("name").unwrap();
    let url = args_add.value_of("url").unwrap();
    let tags = args_add.values_of("tags").unwrap();

    // Check if it already exists.
    if config.bookmarks.contains_key(name) {
        if !args_add.is_present("force") {
            anyhow::bail!(
                "Bookmark already exists with name '{}'. Use '--force' to override.",
                name
            );
        } else {
            config.bookmarks.remove(name);
        }
    }

    // Insert the new bookmark.
    config.bookmarks.insert(
        String::from(name),
        ConfigBookmark {
            url: String::from(url),
            tags: tags.map(String::from).collect(),
        },
    );

    // Save.
    std::fs::write(config_path, serde_yaml::to_string(&config)?)?;
    println!(
        "Added bookmark '{}\t{}'.",
        String::from(name),
        String::from(url)
    );

    Ok(())
}

/// Command to edit a bookmark.
fn command_edit(args: clap::ArgMatches) -> Result<()> {
    // Parse arguments.
    let args_edit = args.subcommand_matches("edit").unwrap();
    let args_edit_name = args_edit.value_of("name").unwrap();

    // Load config.
    let config_path = &std::path::PathBuf::from(
        shellexpand::tilde(args.value_of("config").unwrap()).into_owned(),
    );
    let config = load_config(config_path)?;

    // Check if it already exists.
    if !config.bookmarks.contains_key(args_edit_name) {
        bail!("Bookmark '{}' not found.", args_edit_name);
    }

    // Load in editor.
    // Default '$EDITOR' is assumed to be vim compliant.
    let bookit_edit_command_format = std::result::Result::unwrap_or(
        std::env::var("BOOKIT_EDIT_COMMAND"),
        String::from("$EDITOR \"$BOOKIT_CONFIG_PATH\" \"+/$VIM_BOOKIT_BOOKMARK_NAME\""),
    );
    // Replace fake environment variables.
    let bookit_edit_command = bookit_edit_command_format
        .replace(
            "$BOOKIT_CONFIG_PATH",
            &std::fs::canonicalize(config_path)
                .unwrap()
                .into_os_string()
                .into_string()
                .unwrap(),
        )
        .replace("$BOOKIT_BOOKMARK_NAME", args_edit_name)
        // Find string for vim needs to  escape slashes properly.
        .replace(
            "$VIM_BOOKIT_BOOKMARK_NAME",
            &args_edit_name.replace('/', "\\/"),
        );
    log::info!("Command: {}", bookit_edit_command);

    // Replace environment variables.
    let bookit_edit_command_expanded = shellexpand::env(&bookit_edit_command).unwrap();
    log::info!("Command Expanded: {}", bookit_edit_command_expanded);

    // Quote environment command.
    let bookit_edit_command_quoted_split = shlex::split(&bookit_edit_command_expanded).unwrap();
    let refs_bookit_edit_command_quoted_split: Vec<&str> = bookit_edit_command_quoted_split
        .iter()
        .map(AsRef::as_ref)
        .collect();
    let bookit_edit_command_quoted = shlex::join(refs_bookit_edit_command_quoted_split);
    log::info!("Command Quoted: {}", bookit_edit_command_quoted);

    // Quote any arguments correctly.
    let bookit_edit_command_parts: Vec<String> = shlex::split(&bookit_edit_command_quoted).unwrap();
    log::info!("Command Expanded Parts: {:?}", bookit_edit_command_parts);

    // Run the editor with our arguments.
    std::process::Command::new(&bookit_edit_command_parts[0])
        .args(&bookit_edit_command_parts[1..])
        .status()
        .expect("Unable to edit file");

    // Success.
    println!("Edited bookmark '{}'.", String::from(args_edit_name),);

    Ok(())
}

/// Command to delete a bookmark.
fn command_delete(args: clap::ArgMatches) -> Result<()> {
    // Parse arguments.
    let args_delete = args.subcommand_matches("delete").unwrap();
    let args_delete_name = args_delete.value_of("name").unwrap();

    // Load config.
    let config_path = &std::path::PathBuf::from(
        shellexpand::tilde(args.value_of("config").unwrap()).into_owned(),
    );
    let mut config = load_config(config_path)?;

    // Check if it already exists.
    if !config.bookmarks.contains_key(args_delete_name) {
        bail!("Bookmark doesn't exist with name '{}'.", args_delete_name);
    }

    // Remove bookmark.
    config.bookmarks.remove(args_delete_name);

    // Save.
    std::fs::write(config_path, serde_yaml::to_string(&config)?)?;
    println!("Deleted bookmark '{}'.", String::from(args_delete_name),);

    Ok(())
}

/// Loads a bookit configuration file.
fn load_config(config_path: &std::path::PathBuf) -> Result<Config> {
    // Load config file.
    let contents = std::fs::read_to_string(config_path)
        .with_context(|| format!("No config found at '{}'.", config_path.display()))?;

    // Parse config file.
    let config: Config = serde_yaml::from_str(&contents)?;

    Ok(config)
}
