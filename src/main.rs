use anyhow::{Context, Result};
use dirs;
use std;

/*
 * Bookmarks manager.
 * - Store
 *   - Name
 *   - Url
 *   - Tags
 * - View
 * - Retrieve & open
 */
fn main() {
    let args: Vec<String> = std::env::args().collect();

    // Run the application.
    if let Err(e) = run() {
        println!("{}", e);
        std::process::exit(2);
    }
    std::process::exit(0);
}

fn run() -> Result<()> {
    let home_path = dirs::home_dir().with_context(|| format!("Could not find home directory."))?;
    let bookmarks_path = &home_path.join(".docket");
    let contents = std::fs::read_to_string(bookmarks_path)
        .with_context(move || format!("No bookmarks found at '{}'.", bookmarks_path.display()))?;
    println!("{}", contents);

    return Ok(());
}
