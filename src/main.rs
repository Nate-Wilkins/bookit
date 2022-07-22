use anyhow::{Context, Result};
use clap::{App, Arg, SubCommand};
use regex::Regex;
use serde::{Deserialize, Serialize};
use shellexpand;
use std;
use std::collections::BTreeMap;
use std::str;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Config {
    bookmarks: BTreeMap<String, ConfigBookmark>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct ConfigBookmark {
    url: String,
    tags: Vec<String>,
}

/*
 * Bookmarks manager.
 */
fn main() {
    // Command line interface.
    let args = App::new("docket")
        .version("1.0")
        .author("Nathaniel Wilkins <nate-wilkins@code-null.com>")
        .about("A bookmarks manager!")
        .arg(
            Arg::with_name("config")
                .long("config")
                .required(false)
                .default_value("~/.docket")
                .help("configuration file to use"),
        )
        .subcommand(
            SubCommand::with_name("view").about("view bookmarks").arg(
                Arg::with_name("include-icon")
                    .short("ii")
                    .long("include-icon")
                    .required(false)
                    .takes_value(false)
                    .help("include icon for bookmarks"),
            ),
        )
        .subcommand(SubCommand::with_name("list-tags").about("lists all tags recorded"))
        .subcommand(
            SubCommand::with_name("add")
                .about("add a new bookmark")
                .arg(
                    Arg::with_name("name")
                        .short("n")
                        .long("name")
                        .required(true)
                        .takes_value(true)
                        .help("name of the bookmark"),
                )
                .arg(
                    Arg::with_name("url")
                        .short("u")
                        .long("url")
                        .required(true)
                        .takes_value(true)
                        .help("url of the bookmark"),
                )
                .arg(
                    Arg::with_name("tags")
                        .short("t")
                        .long("tags")
                        .required(true)
                        .multiple(true)
                        .takes_value(true)
                        .help("tags of the bookmark"),
                )
                .arg(
                    Arg::with_name("force")
                        .long("force")
                        .required(false)
                        .takes_value(false)
                        .help("override a bookmark if one already exists"),
                ),
        )
        .subcommand(
            SubCommand::with_name("edit").about("edit a bookmark").arg(
                Arg::with_name("name")
                    .short("n")
                    .long("name")
                    .required(true)
                    .takes_value(true)
                    .help("name of the bookmark"),
            ),
        )
        .subcommand(
            SubCommand::with_name("delete")
                .about("delete a bookmark")
                .arg(
                    Arg::with_name("name")
                        .short("n")
                        .long("name")
                        .required(true)
                        .takes_value(true)
                        .help("name of the bookmark"),
                ),
        )
        .get_matches();

    // Run the application.
    if let Err(e) = run(args) {
        eprintln!("{}", e);
        std::process::exit(2);
    }
    std::process::exit(0);
}

/*
 * Run application according to command line interface arguments.
 */
fn run(args: clap::ArgMatches) -> Result<()> {
    if let Some(_) = args.subcommand_matches("view") {
        command_view(args)?;
    } else if let Some(_) = args.subcommand_matches("list-tags") {
        command_list_tags()?;
    } else if let Some(_) = args.subcommand_matches("add") {
        command_add(args)?;
    } else if let Some(_) = args.subcommand_matches("edit") {
        command_edit(args)?;
    } else if let Some(_) = args.subcommand_matches("delete") {
        command_delete()?;
    }

    return Ok(());
}

const REGEX_HOSTNAME: &str = r"^([^:]*://)([^/]*)/?.*?$";

/*
 * Command to view bookmarks.
 */
fn command_view(args: clap::ArgMatches) -> Result<()> {
    // Load config.
    let config_path = &std::path::PathBuf::from(
        shellexpand::tilde(args.value_of("config").unwrap()).into_owned(),
    );
    let config = load_config(config_path)?;

    // Print out every bookmark with corresponding context.
    for bookmark in config.bookmarks.iter() {
        // TODO: Should check for '--include-icon'.
        let re = Regex::new(REGEX_HOSTNAME).unwrap();
        let caps = re.captures(&bookmark.1.url).unwrap();
        let hostname = caps.get(2).unwrap().as_str();

        println!(
            "{}\t{}\t{}\t\0icon\x1f{}",
            bookmark.0,
            bookmark.1.tags.join(","),
            bookmark.1.url,
            hostname
        );
    }
    return Ok(());
}

/*
 * Command to list out bookmark tags.
 */
fn command_list_tags() -> Result<()> {
    // TODO@nw:
    return Ok(());
}

/*
 * Command to add a bookmark.
 */
fn command_add(args: clap::ArgMatches) -> Result<()> {
    let args_add = args.subcommand_matches("add").unwrap();

    // Load config.
    let config_path = &std::path::PathBuf::from(
        shellexpand::tilde(args.value_of("config").unwrap()).into_owned(),
    );
    let mut config = load_config(config_path)?;

    // Add bookmark.
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
            tags: tags.map(|t| String::from(t)).collect(),
        },
    );

    // Save.
    std::fs::write(config_path, serde_yaml::to_string(&config)?)?;

    return Ok(());
}

/*
 * Command to edit a bookmark.
 */
fn command_edit(args: clap::ArgMatches) -> Result<()> {
    let args_edit = args.subcommand_matches("edit").unwrap();

    // Load config.
    let config_path = &std::path::PathBuf::from(
        shellexpand::tilde(args.value_of("config").unwrap()).into_owned(),
    );
    let mut config = load_config(config_path)?;

    // Pull from args the bookmark being edited.
    let name = args_edit.value_of("name").unwrap();

    // Check if it already exists.
    if config.bookmarks.contains_key(name) {
        // TODO: Error here.
    }

    // TODO: Load in editor.
    // let editor = std::env::var("EDITOR").unwrap();
    // let mut file_path = std::env::temp_dir();
    // file_path.push("editable");
    // std::fs::File::create(&file_path).expect("Could not create file");

    // std::process::Command::new(editor)
    //     .arg(&file_path)
    //     .status()
    //     .expect("Something went wrong");

    return Ok(());
}

/*
 * Command to delete a bookmark.
 */
fn command_delete() -> Result<()> {
    // - TODO@nw: Delete
    //            docket delete --name "Some Bookmark"
    return Ok(());
}

/*
 * Loads a docket configuration file.
 */
fn load_config(config_path: &std::path::PathBuf) -> Result<Config> {
    // Load config file.
    let contents = std::fs::read_to_string(config_path)
        .with_context(|| format!("No config found at '{}'.", config_path.display()))?;

    // Parse config file.
    let config: Config = serde_yaml::from_str(&contents)?;

    return Ok(config);
}
