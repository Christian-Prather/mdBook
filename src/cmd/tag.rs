use super::command_prelude::*;
use crate::get_book_dir;
use copy_dir::copy_dir;
use mdbook::errors::Result;
use mdbook::MDBook;
use std::path::PathBuf;

// Create clap subcommand arguments
pub fn make_subcommand() -> Command {
    Command::new("tag")
        .about("Tags the current book with a version tag")
        .arg_tag()
        .arg_dest_dir()
        .arg_root_dir()
}

// Build command implementation
pub fn execute(args: &ArgMatches) -> Result<()> {
    let book_dir = get_book_dir(args);
    let mut book = MDBook::load(book_dir)?;

    if let Some(dest_dir) = args.get_one::<PathBuf>("dest-dir") {
        book.config.build.build_dir = dest_dir.into();
    }

    book.build()?;

    // FIXME: What's the right behaviour if we don't use the HTML renderer?
    let path = book.root.join("versions");
    let tag_value = args.get_one::<String>("tag").expect("No tag version given");
    if !path.exists() {
        error!("No versions dir available to open");
        std::process::exit(1);
    }

    let version_path = path.join(tag_value);

    copy_dir(book.config.build.build_dir, version_path)?;
    Ok(())
}
