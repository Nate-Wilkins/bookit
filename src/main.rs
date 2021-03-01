use anyhow::{Context, Result};
use dirs;
use std;
use yaml_rust::YamlLoader;

/*
 * Bookmarks manager.
 * - View
 * - TODO@nw: Add
 *            docket add --name "Some Bookmark" --url "https://google.com" --tags search --tags nw
 * - TODO@nw: Delete
 */
fn main() {
    // let args: Vec<String> = std::env::args().collect();

    // Run the application.
    if let Err(e) = run() {
        eprintln!("{}", e);
        std::process::exit(2);
    }
    std::process::exit(0);
}

fn run() -> Result<()> {
    // Load bookmarks file.
    let home_path = dirs::home_dir().with_context(|| format!("Could not find home directory."))?;
    let bookmarks_path = &home_path.join(".docket");
    let bookmarks_contents = &std::fs::read_to_string(bookmarks_path)
        .with_context(|| format!("No bookmarks found at '{}'.", bookmarks_path.display()))?;

    // Parse bookmarks file.
    let bookmarks_parsed_docs = YamlLoader::load_from_str(bookmarks_contents)?;
    let bookmarks_parsed_doc = &bookmarks_parsed_docs[0];

    // Pull out bookmarks.
    let bookmarks = &bookmarks_parsed_doc["bookmarks"];
    if bookmarks.is_badvalue() {
        anyhow::bail!(
            "No 'bookmarks' key found in '{}'.",
            bookmarks_path.display()
        );
    }
    let bookmarks_hash = &bookmarks.as_hash().with_context(|| {
        format!(
            "No 'bookmarks' hash map found in '{}'.",
            bookmarks_path.display(),
        )
    })?;

    // Print out every bookmark with corresponding context.
    for bookmark in bookmarks_hash.iter() {
        let name = bookmark.0.as_str().with_context(|| {
            format!(
                "The 'bookmarks' hash map is malformed in '{}'.",
                bookmarks_path.display(),
            )
        })?;
        let url = bookmark.1["url"].as_str().with_context(|| {
            format!(
                "Bookmark '{}' malformed at 'url' key in '{}'.",
                name,
                bookmarks_path.display()
            )
        })?;
        let tags = bookmark.1["tags"].as_vec().with_context(|| {
            format!(
                "Bookmark '{}' malformed at 'tags' key in '{}'.",
                name,
                bookmarks_path.display()
            )
        })?;
        let mut tags_parsed = vec![];
        for tag in tags {
            let tag_parsed = tag.as_str().with_context(|| {
                format!(
                    "Bookmark '{}' expected strings at 'tags' key in '{}'.",
                    name,
                    bookmarks_path.display()
                )
            })?;
            tags_parsed.push(tag_parsed);
        }

        println!("{}\t{}\t{}", name, tags_parsed.join(","), url);
    }

    return Ok(());
}
