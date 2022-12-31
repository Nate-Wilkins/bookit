use anyhow::{anyhow, Context, Result};
use regex::Regex;

#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
struct Config {
    bookmarks: std::collections::BTreeMap<String, ConfigBookmark>,
}

#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
struct ConfigBookmark {
    url: String,
    tags: Vec<String>,
}

/*
 * Bookmarks manager.
 */
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

/*
 * Create the application command line interface.
 */
fn create_application() -> clap::App<'static, 'static> {
    return clap::App::new("docket")
        .bin_name("docket")
        .version("1.0")
        .author("Nathaniel Wilkins <nate-wilkins@code-null.com>")
        .about("Fast and simple bookmark manager for your operating system.")
        .arg(
            clap::Arg::with_name("config")
                .global(true)
                .long("config")
                .required(false)
                .env("DOCKET_CONFIG_PATH")
                .default_value("~/.docket")
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
                    clap::Arg::with_name("include-icon")
                        .short("ii")
                        .long("include-icon")
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

/*
 * Initializes the application logger.
 */
fn initialize_logger() {
    // TODO: Support `--verbosity`.
    let env = env_logger::Env::default().filter("DOCKET_LOG_LEVEL");

    return env_logger::Builder::from_env(env)
        .target(env_logger::Target::Stdout)
        .init();
}

/*
 * Run application according to command line interface arguments.
 */
fn run(args: clap::ArgMatches) -> Result<()> {
    if let Some(_) = args.subcommand_matches("completions") {
        command_completions(args)?;
    } else if let Some(_) = args.subcommand_matches("view") {
        command_view(args)?;
    } else if let Some(_) = args.subcommand_matches("list-tags") {
        command_list_tags()?;
    } else if let Some(_) = args.subcommand_matches("add") {
        command_add(args)?;
    } else if let Some(_) = args.subcommand_matches("edit") {
        command_edit(args)?;
    } else if let Some(_) = args.subcommand_matches("delete") {
        command_delete(args)?;
    }

    return Ok(());
}

const REGEX_HOSTNAME: &str = r"^([^:]*://)([^/]*)/?.*?$";

/*
 * Output completions of a specific type to STDOUT.
 */
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
        anyhow::bail!("Completion type '{}' not supported.", completion_type);
    }

    return Ok(());
}

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
        // TODO: Should really gracefully fail here and log which record could not be parsed.
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
    let config = load_config(config_path)?;

    // Pull from args the bookmark being edited.
    let name = args_edit.value_of("name").unwrap();

    // Check if it already exists.
    if !config.bookmarks.contains_key(name) {
        return Err(anyhow!("Bookmark '{}' not found.", name));
    }

    // Load in editor.
    let docket_edit_command_format = std::env::var("DOCKET_EDIT_COMMAND").unwrap_or(String::from(
        "'$EDITOR' '$DOCKET_CONFIG_PATH' '+/$DOCKET_BOOKMARK_NAME'",
    ));
    let docket_edit_command = docket_edit_command_format
        .replace(
            "$DOCKET_CONFIG_PATH",
            &std::fs::canonicalize(&config_path)
                .unwrap()
                .into_os_string()
                .into_string()
                .unwrap(),
        )
        .replace("$DOCKET_BOOKMARK_NAME", name);
    let docket_edit_command_expanded =
        String::from(shellexpand::env(&docket_edit_command).unwrap());
    let docket_edit_command_parts: Vec<String> =
        shlex::split(&docket_edit_command_expanded).unwrap();
    log::info!("Running command: {:?}", docket_edit_command_parts);
    std::process::Command::new(&docket_edit_command_parts[0])
        .args(&docket_edit_command_parts[1..])
        .status()
        .expect("Unable to edit file");

    return Ok(());
}

/*
 * Command to delete a bookmark.
 */
fn command_delete(args: clap::ArgMatches) -> Result<()> {
    let args_delete = args.subcommand_matches("delete").unwrap();

    // Load config.
    let config_path = &std::path::PathBuf::from(
        shellexpand::tilde(args.value_of("config").unwrap()).into_owned(),
    );
    let mut config = load_config(config_path)?;

    // Parse bookmark details.
    let name = args_delete.value_of("name").unwrap();

    // Check if it already exists.
    if !config.bookmarks.contains_key(name) {
        anyhow::bail!("Bookmark doesn't exist with name '{}'.", name);
    }

    // Remove bookmark.
    config.bookmarks.remove(name);

    // Save.
    std::fs::write(config_path, serde_yaml::to_string(&config)?)?;

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
