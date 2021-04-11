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

    rnm::rename_files(".", &args)
}
