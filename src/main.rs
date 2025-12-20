use env_logger::Builder;
use log::LevelFilter;
use rnm::CliArguments;

fn main() -> std::io::Result<()> {
    Builder::from_default_env()
        .format_timestamp(None)
        .format_module_path(false)
        .filter(None, LevelFilter::Info)
        .init();

    let args: CliArguments = argh::from_env();

    // IMPROVEMENT: Consider making the path configurable via CLI argument
    // IMPROVEMENT: Consider adding a --dry-run flag to preview changes before applying
    // IMPROVEMENT: Consider better error messages (e.g., if path doesn't exist)
    rnm::rename_files(".", &args)
}
