use log::info;
use std::fs;
use structopt::StructOpt;

mod utils;

#[derive(StructOpt)]
pub struct CliArguments {
    from: String,
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
