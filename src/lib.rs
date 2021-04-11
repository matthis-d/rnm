use argh::FromArgs;
use log::info;
use std::fs;

mod utils;

/// Replaces filenames using regexes
#[derive(FromArgs)]
pub struct CliArguments {
    /// regex of filenames to replace
    #[argh(positional)]
    from: String,

    /// replacement to apply with $1, $2, etc. as values catched in regex.
    #[argh(positional)]
    to: String,
}

pub fn rename_files(path: &str, args: &CliArguments) -> std::io::Result<()> {
    info!("from {} into {}", args.from, args.to);

    for entry in utils::files::get_file_entries(path) {
        if let Some(pathname) = entry.path().to_str() {
            let output = utils::string::replace_name(&pathname, &args.from, &args.to);
            if output != pathname {
                fs::rename(pathname, output)?;
            }
        }
    }
    Ok(())
}
