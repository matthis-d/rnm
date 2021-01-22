use env_logger::Builder;
use log::LevelFilter;
use structopt::StructOpt;

fn main() -> std::io::Result<()> {
    Builder::from_default_env()
        .format_timestamp(None)
        .format_module_path(false)
        .filter(None, LevelFilter::Info)
        .init();

    let args = rnm::CliArguments::from_args();

    rnm::rename_files(".", &args)
}
