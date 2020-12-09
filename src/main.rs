use env_logger::Builder;
use log::{info, LevelFilter};
use std::fs;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    from: String,
    to: String,
}

fn main() -> std::io::Result<()> {
    Builder::from_default_env()
        .format_timestamp(None)
        .format_module_path(false)
        .filter(None, LevelFilter::Info)
        .init();

    let args = Cli::from_args();
    info!("from {} into {}", args.from, args.to);

    for entry in get_file_entries(".") {
        if let Some(pathname) = entry.path().to_str() {
            let output = rnm::replace_name(&pathname, &args.from, &args.to);
            if output != pathname {
                fs::rename(pathname, output)?;
            }
        }
    }
    Ok(())
}

fn get_file_entries(pathname: &str) -> Vec<fs::DirEntry> {
    fs::read_dir(pathname)
        .unwrap()
        // Keep readable entries
        .filter_map(Result::ok)
        // Keep entries with a redable file type
        .filter(|entry| Result::is_ok(&entry.file_type()))
        // Keep entries of file type
        .filter(|entry| entry.file_type().unwrap().is_file())
        // Return an iterable
        .collect()
}
